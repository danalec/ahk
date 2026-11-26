"""As referencias RIP-relative a um endereco, dentro de um binario de PE.

    python scripts/find-rip-references.py <ficheiro> <endereco> [...]
"""
import struct
import sys


def the_sections(data):
    """As seccoes do PE, com o endereco virtual e o ficheiro."""
    offset = struct.unpack_from("<I", data, 0x3C)[0]
    assert data[offset:offset + 4] == b"PE\0\0"
    machine, count = struct.unpack_from("<HH", data, offset + 4)
    optional = offset + 24
    magic = struct.unpack_from("<H", data, optional)[0]
    image_base = struct.unpack_from("<Q", data, optional + 24)[0]
    size_of_headers = struct.unpack_from("<I", data, optional + 60)[0]
    sections = []
    table = optional + (240 if magic == 0x20B else 224)
    for number in range(count):
        entry = table + number * 40
        name = data[entry:entry + 8].rstrip(b"\0").decode("ascii", "replace")
        virtual_size, virtual_address, raw_size, raw_offset = struct.unpack_from(
            "<IIII", data, entry + 8
        )
        sections.append((name, virtual_address, virtual_size, raw_offset, raw_size))
    return image_base, size_of_headers, sections


def the_references(data, image_base, sections, wanted):
    """Os lugares do ficheiro que apontam para os enderecos queridos."""
    hits = []
    for name, virtual_address, virtual_size, raw_offset, raw_size in sections:
        if not name.startswith(".text"):
            continue
        end = min(raw_size, len(data) - raw_offset)
        for at in range(0, end - 4):
            displacement = struct.unpack_from("<i", data, raw_offset + at)[0]
            target = image_base + virtual_address + at + 4 + displacement
            if target in wanted:
                hits.append((name, image_base + virtual_address + at, target))
    return hits


def main():
    path = sys.argv[1]
    wanted = {int(word, 0) for word in sys.argv[2:]}
    data = open(path, "rb").read()
    image_base, _, sections = the_sections(data)
    print(f"a base da imagem: {image_base:#x}")
    for name, virtual_address, virtual_size, raw_offset, raw_size in sections:
        print(f"  {name:10} {image_base + virtual_address:#x} {virtual_size:#x}")
    for name, address, target in the_references(data, image_base, sections, wanted):
        print(f"  {name} {address:#x} -> {target:#x}")


if __name__ == "__main__":
    main()
