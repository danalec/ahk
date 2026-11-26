#!/usr/bin/env python3
"""The icons of the tree, out of the SVG of the tree.

The icon of the tree is one drawing (`icon.svg`, beside this script), and
every resource of it comes from that drawing:

- the icon of Windows (`icon.ico`), which the build of the runner
  compiles as a resource and the link puts inside the binary;
- the icons of the theme of a desktop
  (`icons/hicolor/<size>x<size>/apps/ahk.png`), one per size;
- the scalable icon of the theme (`icons/hicolor/scalable/apps/ahk.svg`),
  which is the drawing itself.

The sizes of the theme are the ones of the freedesktop specification:
16, 24, 32, 48, 64, 128, 256 and 512.

The drawing is rasterized here: the shapes of the SVG are polygons (the
arcs are flattened into segments), and every shape is filled with a
scanline that samples each pixel four times by four, so the edge of the
shape has the anti-aliasing of a real renderer.

Usage:

    python resources/icons-from-the-svg.py [--check]

`--check` fails when the resources of the tree are not the ones the
drawing gives, and it writes nothing.
"""

import math
import os
import struct
import sys
import zlib
import xml.etree.ElementTree as ET

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
THE_SVG = os.path.join(HERE, 'icon.svg')
THE_ICO = os.path.join(HERE, 'icon.ico')
THE_FOLDER = os.path.join(HERE, 'icons', 'hicolor')
THE_NAME = 'ahk.png'
THE_SCALABLE = os.path.join(THE_FOLDER, 'scalable', 'apps', 'ahk.svg')

# The sizes of the theme of the freedesktop specification.
THE_SIZES = [16, 24, 32, 48, 64, 128, 256, 512]

# The sizes the icon of Windows carries.
THE_SIZES_OF_THE_ICO = [16, 24, 32, 48, 64, 128, 256]

# The samples of a pixel, by side: four by four is sixteen samples, and
# the edge of the shape is anti-aliased with them.
THE_SAMPLES = 4

# The number of segments of an arc: the arc of the drawing is a half
# circle, and sixteen segments of it are enough for a pixel of 512.
THE_SEGMENTS = 32


class TheShape:
    """A shape of the drawing: a polygon, and the colour of it."""

    def __init__(self, points, colour):
        self.points = points
        self.colour = colour

    def the_rows(self):
        """The first and the last row of the shape, in the units of the drawing."""
        ys = [point[1] for point in self.points]
        return min(ys), max(ys)


def the_colour_of(text):
    """The colour of a fill of the SVG: `#rrggbb`."""
    text = (text or '').strip()
    if not text.startswith('#') or len(text) != 7:
        raise SystemExit('the drawing has a fill that is not a colour: %r' % text)
    return (int(text[1:3], 16), int(text[3:5], 16), int(text[5:7], 16))


def the_arc_of_a_circle(x0, y0, rx, ry, large, sweep, x1, y1):
    """The points of an elliptical arc, from the endpoint form of the SVG."""
    if rx == 0 or ry == 0:
        return [(x1, y1)]
    rx, ry = abs(rx), abs(ry)
    # The centre of the arc, from the formula of the specification.
    dx2, dy2 = (x0 - x1) / 2.0, (y0 - y1) / 2.0
    factor = (rx * rx * ry * ry - rx * rx * dy2 * dy2 - ry * ry * dx2 * dx2)
    factor /= (rx * rx * dy2 * dy2 + ry * ry * dx2 * dx2)
    if factor < 0:
        factor = 0
    factor = math.sqrt(factor)
    if large == sweep:
        factor = -factor
    cx = factor * rx * dy2 / ry + (x0 + x1) / 2.0
    cy = -factor * ry * dx2 / rx + (y0 + y1) / 2.0
    # The angles of the ends.
    def the_angle(ux, uy, vx, vy):
        dot = ux * vx + uy * vy
        length = math.sqrt((ux * ux + uy * uy) * (vx * vx + vy * vy))
        if length == 0:
            return 0.0
        value = max(-1.0, min(1.0, dot / length))
        angle = math.acos(value)
        if ux * vy - uy * vx < 0:
            angle = -angle
        return angle

    ux, uy = (x0 - cx) / rx, (y0 - cy) / ry
    vx, vy = (x1 - cx) / rx, (y1 - cy) / ry
    start = the_angle(1.0, 0.0, ux, uy)
    end = the_angle(1.0, 0.0, vx, vy)
    delta = end - start
    if sweep == 0 and delta > 0:
        delta -= 2 * math.pi
    elif sweep == 1 and delta < 0:
        delta += 2 * math.pi
    points = []
    for step in range(1, THE_SEGMENTS + 1):
        angle = start + delta * step / THE_SEGMENTS
        points.append((cx + rx * math.cos(angle), cy + ry * math.sin(angle)))
    return points


def the_points_of_a_path(text):
    """The polygon of a path of the SVG.

    The paths of the drawing are made of the moves, the lines and the
    arcs: `M`, `H`, `V`, `L`, `A` and `Z`. A command that is not one of
    them is refused with the reason.
    """
    points = []
    at = 0
    command = None
    after_a_move = False

    def the_number():
        nonlocal at
        while at < len(text) and text[at] in ' ,\t\n\r':
            at += 1
        start = at
        while at < len(text) and (text[at].isdigit() or text[at] in '.-+'):
            at += 1
        if start == at:
            raise SystemExit('the drawing has a path with a number that is not one: %r' % text)
        return float(text[start:at])

    while at < len(text):
        while at < len(text) and text[at] in ' ,\t\n\r':
            at += 1
        if at >= len(text):
            break
        if text[at].isalpha():
            command = text[at]
            at += 1
            after_a_move = False
        elif command is None and after_a_move:
            # A pair of numbers after a moveto is a lineto: the rule of
            # the SVG.
            command = 'L'
        if command is None:
            raise SystemExit('the drawing has a path that does not begin: %r' % text)
        if command == 'M':
            x, y = the_number(), the_number()
            points.append((x, y))
            command = None
            after_a_move = True
        elif command == 'L':
            x, y = the_number(), the_number()
            points.append((x, y))
        elif command == 'H':
            x = the_number()
            points.append((x, points[-1][1]))
        elif command == 'V':
            y = the_number()
            points.append((points[-1][0], y))
        elif command == 'A':
            rx, ry, rotation, large, sweep = (the_number(), the_number(), the_number(),
                                              the_number(), the_number())
            x, y = the_number(), the_number()
            if rotation != 0:
                raise SystemExit('the drawing has an arc that is not a circle')
            points.extend(the_arc_of_a_circle(points[-1][0], points[-1][1], rx, ry,
                                               int(large), int(sweep), x, y))
        elif command == 'Z':
            if points and points[0] != points[-1]:
                points.append(points[0])
        else:
            raise SystemExit('the drawing has a command that is not one of them: %r' % command)
    return points


def the_points_of_a_rect(x, y, width, height, rx):
    """The polygon of a rectangle with rounded corners."""
    rx = min(rx, width / 2.0, height / 2.0)
    if rx <= 0:
        return [(x, y), (x + width, y), (x + width, y + height), (x, y + height), (x, y)]
    ry = rx
    points = []
    corners = [
        (x + width - rx, y + rx, -math.pi / 2, 0.0),
        (x + width - rx, y + height - rx, 0.0, math.pi / 2),
        (x + rx, y + height - rx, math.pi / 2, math.pi),
        (x + rx, y + rx, math.pi, 3 * math.pi / 2),
    ]
    for cx, cy, start, end in corners:
        points.append((cx + rx * math.cos(start), cy + rx * math.sin(start)))
        for step in range(1, THE_SEGMENTS + 1):
            angle = start + (end - start) * step / THE_SEGMENTS
            points.append((cx + rx * math.cos(angle), cy + ry * math.sin(angle)))
    points.append(points[0])
    return points


def the_shapes_of_the_drawing():
    """The shapes of the drawing, in the order of the painter."""
    tree = ET.parse(THE_SVG)
    root = tree.getroot()
    view = root.get('viewBox')
    if not view:
        raise SystemExit('the drawing has no viewBox')
    shapes = []
    for node in root:
        tag = node.tag.split('}')[-1]
        colour = the_colour_of(node.get('fill'))
        if tag == 'rect':
            points = the_points_of_a_rect(float(node.get('x', 0)), float(node.get('y', 0)),
                                          float(node.get('width')), float(node.get('height')),
                                          float(node.get('rx', 0)))
            shapes.append(TheShape(points, colour))
        elif tag == 'path':
            points = the_points_of_a_path(node.get('d'))
            shapes.append(TheShape(points, colour))
        elif tag in ('title', 'desc', 'comment'):
            continue
        else:
            raise SystemExit('the drawing has a shape that is not one of them: %r' % tag)
    return shapes, [float(number) for number in view.split()]


def the_coverage_of_a_shape(shape, size, scale):
    """The coverage of a shape: a count of samples by pixel."""
    counts = [0] * (size * size)
    points = shape.points
    first, last = shape.the_rows()
    row0 = max(0, int(math.floor(first * scale)))
    row1 = min(size - 1, int(math.ceil(last * scale)))
    edges = list(zip(points, points[1:] + points[:1]))
    for row in range(row0, row1 + 1):
        for sample in range(THE_SAMPLES):
            y = (row + (sample + 0.5) / THE_SAMPLES) / scale
            crossings = []
            for (x0, y0), (x1, y1) in edges:
                if (y0 <= y < y1) or (y1 <= y < y0):
                    crossings.append(x0 + (y - y0) * (x1 - x0) / (y1 - y0))
            crossings.sort()
            for index in range(0, len(crossings) - 1, 2):
                left = crossings[index] * scale
                right = crossings[index + 1] * scale
                first_column = max(0, int(math.floor(left)))
                last_column = min(size - 1, int(math.ceil(right)))
                for column in range(first_column, last_column + 1):
                    for other in range(THE_SAMPLES):
                        x = column + (other + 0.5) / THE_SAMPLES
                        if left <= x < right:
                            counts[row * size + column] += 1
    return counts


def the_image_of_the_drawing(shapes, view, size):
    """The image of the drawing at this size, as rows of RGBA."""
    scale = size / view[2]
    red = [0.0] * (size * size)
    green = [0.0] * (size * size)
    blue = [0.0] * (size * size)
    alpha = [0.0] * (size * size)
    for shape in shapes:
        counts = the_coverage_of_a_shape(shape, size, scale)
        colour = shape.colour
        for index in range(size * size):
            coverage = counts[index] / float(THE_SAMPLES * THE_SAMPLES)
            if coverage <= 0:
                continue
            # The shape is put over what is there: the source over.
            rest = 1.0 - coverage
            red[index] = colour[0] * coverage + red[index] * rest
            green[index] = colour[1] * coverage + green[index] * rest
            blue[index] = colour[2] * coverage + blue[index] * rest
            alpha[index] = coverage + alpha[index] * rest
    rows = []
    for row in range(size):
        line = bytearray()
        for column in range(size):
            index = row * size + column
            a = alpha[index]
            if a <= 0:
                line += bytes((0, 0, 0, 0))
                continue
            line += bytes((int(round(red[index] / a)), int(round(green[index] / a)),
                           int(round(blue[index] / a)), int(round(a * 255))))
        rows.append(bytes(line))
    return rows


def the_chunk_of_a_png(kind, data):
    return (struct.pack('>I', len(data)) + kind + data +
            struct.pack('>I', zlib.crc32(kind + data) & 0xffffffff))


def the_png_of_the_rows(width, height, rows):
    """The PNG of the rows of RGBA."""
    raw = b''.join(b'\x00' + row for row in rows)
    header = struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)
    return (b'\x89PNG\r\n\x1a\n' + the_chunk_of_a_png(b'IHDR', header) +
            the_chunk_of_a_png(b'IDAT', zlib.compress(raw, 9)) +
            the_chunk_of_a_png(b'IEND', b''))


def the_dib_of_the_rows(width, height, rows):
    """The DIB of an icon of Windows: the pixels and the mask of transparency."""
    header = struct.pack('<IiiHHIIiiII', 40, width, height * 2, 1, 32, 0, 0, 0, 0, 0, 0)
    pixels = bytearray()
    stride = ((width + 31) // 32) * 4
    for row in range(height - 1, -1, -1):
        line = rows[row]
        for column in range(width):
            red, green, blue, alpha = line[column * 4:column * 4 + 4]
            pixels += bytes((blue, green, red, alpha))
    mask = bytearray()
    for row in range(height - 1, -1, -1):
        line = rows[row]
        bits = bytearray(stride)
        for column in range(width):
            alpha = line[column * 4 + 3]
            if alpha == 0:
                bits[column // 8] |= 0x80 >> (column % 8)
        mask += bits
    return header + bytes(pixels) + bytes(mask)


def the_ico_of_the_images(images):
    """The icon of Windows out of the images of the drawing."""
    entries = []
    for size, image in images:
        if size >= 256:
            data = the_png_of_the_rows(size, size, image)
        else:
            data = the_dib_of_the_rows(size, size, image)
        entries.append((size, size, data))
    out = bytearray(struct.pack('<HHH', 0, 1, len(entries)))
    at = 6 + 16 * len(entries)
    for width, height, data in entries:
        out += struct.pack('<BBBBHHII', width % 256, height % 256, 0, 0, 1, 32,
                           len(data), at)
        at += len(data)
    for _width, _height, data in entries:
        out += data
    return bytes(out)


def the_scalable_of_the_drawing():
    """The scalable icon of the theme: the drawing, without the size of it."""
    with open(THE_SVG, encoding='utf-8') as handle:
        text = handle.read()
    out = []
    for line in text.split('\n'):
        if line.strip().startswith('<svg'):
            line = line.replace(' width="200"', '').replace(' height="200"', '')
        out.append(line)
    return '\n'.join(out)


def the_file_of_the_size(size):
    return os.path.join(THE_FOLDER, '%dx%d' % (size, size), 'apps', THE_NAME)


def the_written(path, data, check):
    """Write the file, or say that it is out of date."""
    if check:
        if not os.path.isfile(path):
            print('missing: %s' % path)
            return False
        with open(path, 'rb') as handle:
            if handle.read() == data:
                return True
        print('out of date: %s' % path)
        return False
    os.makedirs(os.path.dirname(path), exist_ok=True)
    # The file is written beside and takes the place of the old one: a file
    # that the shell has mapped -- the icon of a folder, for one -- cannot
    # be written in place, and the machine refuses the write.
    temp = path + '.new'
    with open(temp, 'wb') as handle:
        handle.write(data)
    if os.path.exists(path):
        os.remove(path)
    os.replace(temp, path)
    return True


def main(argv):
    check = '--check' in argv
    shapes, view = the_shapes_of_the_drawing()
    images = {}
    for size in sorted(set(THE_SIZES + THE_SIZES_OF_THE_ICO)):
        images[size] = the_image_of_the_drawing(shapes, view, size)
    good = True
    for size in THE_SIZES:
        path = the_file_of_the_size(size)
        data = the_png_of_the_rows(size, size, images[size])
        good = the_written(path, data, check) and good
    good = the_written(THE_SCALABLE, the_scalable_of_the_drawing().encode('utf-8'),
                       check) and good
    good = the_written(THE_ICO, the_ico_of_the_images(
        [(size, images[size]) for size in THE_SIZES_OF_THE_ICO]), check) and good
    if not check:
        print('the icons of the tree: %s' % ', '.join(str(size) for size in THE_SIZES))
    return 0 if good else 1


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
