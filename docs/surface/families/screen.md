# Screen capture and pixel functions

| [English](index.md) | [Español](es/index.md) | [Português (Brasil)](pt-BR/index.md) | [Français](fr/index.md) | [Italiano](it/index.md) | [Polski](pl/index.md) | [Türkçe](tr/index.md) | [Deutsch](de/index.md) | [Русский](ru/index.md) | [日本語](ja/index.md) | [中文（简体）](zh/index.md) | [繁體中文](zh-Hant/index.md) | [한국어](ko/index.md) |

Language: English

The names of the family **Screen capture and pixel functions** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State | Probe |
|---|---|---|---|---|---|---|
| `A_ScreenDPI` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_ScreenHeight` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_ScreenWidth` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `ImageSearch` | AutoHotkey v2.0.19 | command | -- | 7 | value | -- |
| `ImageSearch` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | value | -- |
| `LoadPicture` | AutoHotkey v2.0.19 | command | -- | 1 to 3 | value | -- |
| `LoadPicture` | AutoHotkey v1.1.37.02 | function | -- | 1 to 3 | value | -- |
| `PixelGetColor` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | value | -- |
| `PixelGetColor` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | value | -- |
| `PixelSearch` | AutoHotkey v2.0.19 | command | -- | 7 to 8 | value | -- |
| `PixelSearch` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | value | -- |
| `ScreenDPI` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `ScreenHeight` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `ScreenWidth` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |

## What the tree does with it

### A_ScreenHeight

The height of the screen of the desktop, in pixels.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the height, as a number
- **Errors:** the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

```ahk
FileAppend A_ScreenHeight, "*"
```

### A_ScreenWidth

The width of the screen of the desktop, in pixels.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the width, as a number
- **Errors:** the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

```ahk
FileAppend A_ScreenWidth, "*"
```

### ImageSearch

Looks for a picture on the screen. The region is captured once and the picture is looked for in it, with the variation and the transparent colour.

- **Parameters:** &X, &Y, X1, Y1, X2, Y2, Picture [, Options]
- **Answers:** one when the picture is there, zero when it is not
- **Errors:** a picture that cannot be loaded is refused with 'Parameter #7 of ImageSearch is invalid.'; on a host without `grim` the search fails with the reason of `grim`.

```ahk
ImageSearch &x, &y, 0, 0, 800, 600, "button.png"
```

### PixelGetColor

The colour of one pixel, read from the frame the compositor draws. The box captures a 1x1 region with `grim` and reads the one pixel.

- **Parameters:** X, Y [, Default]
- **Answers:** the colour, as `#rrggbb`
- **Errors:** a colour written as a number is refused with 'is not a colour: write it as #rrggbb'; on a host without `grim` the read fails with the reason of `grim`.

```ahk
colour := PixelGetColor(10, 20)
```

### PixelSearch

Where a colour is on the screen. The region is captured once and walked for the first pixel of that colour. It is the check that works whatever the desktop is.

- **Parameters:** &X, &Y, X1, Y1, X2, Y2, Color
- **Answers:** one when the colour is there, zero when it is not
- **Errors:** the two places are written into the variables of the caller, and they are made blank when the colour is not there; on a host without `grim` the search fails with the reason of `grim`.

```ahk
PixelSearch &x, &y, 0, 0, 100, 100, "#ff0000"
```
