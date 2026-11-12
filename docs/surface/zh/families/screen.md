# 屏幕捕获与像素函数

| [English](../index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

语言: 中文（简体）

三个方言中属于 **屏幕捕获与像素函数** 类的名称，以及这棵树对每个名称给出的回答。分类是脚本思考的领域，这些名称即是它的全部。

| 名称 | 方言 | 种类 | 类 | 参数 | 状态 | 调用探测 |
|---|---|---|---|---|---|---|
| `A_ScreenDPI` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_ScreenHeight` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_ScreenWidth` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `ImageSearch` | AutoHotkey v2.0.19 | command | -- | 7 | 值 | -- |
| `ImageSearch` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | 值 | -- |
| `LoadPicture` | AutoHotkey v2.0.19 | command | -- | 1 to 3 | 值 | -- |
| `LoadPicture` | AutoHotkey v1.1.37.02 | function | -- | 1 to 3 | 值 | -- |
| `PixelGetColor` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | 值 | -- |
| `PixelGetColor` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | 值 | -- |
| `PixelSearch` | AutoHotkey v2.0.19 | command | -- | 7 to 8 | 值 | -- |
| `PixelSearch` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | 值 | -- |
| `ScreenDPI` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `ScreenHeight` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `ScreenWidth` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |

## 这棵树对它做什么

### A_ScreenHeight

桌面屏幕的高度，单位是像素。

- **参数:** nothing; it is a variable, not a call
- **返回:** 高度，作为数字
- **错误:** the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

```ahk
FileAppend A_ScreenHeight, "*"
```

### A_ScreenWidth

桌面屏幕的宽度，单位是像素。

- **参数:** nothing; it is a variable, not a call
- **返回:** 宽度，作为数字
- **错误:** the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

```ahk
FileAppend A_ScreenWidth, "*"
```

### ImageSearch

在屏幕上寻找一张图片。区域只截取一次，然后在其中查找这张图片，带上容差和透明色。

- **参数:** &X, &Y, X1, Y1, X2, Y2, Picture [, Options]
- **返回:** 找到图片时为 1，否则为 0
- **错误:** a picture that cannot be loaded is refused with 'Parameter #7 of ImageSearch is invalid.'; on a host without `grim` the search fails with the reason of `grim`.

```ahk
ImageSearch &x, &y, 0, 0, 800, 600, "button.png"
```

### PixelGetColor

读取一个像素的颜色，读的是合成器绘制的画面。后端用 `grim` 截取 1x1 区域，再读出那一个像素。

- **参数:** X, Y [, Default]
- **返回:** 颜色，格式为 `#rrggbb`
- **错误:** a colour written as a number is refused with 'is not a colour: write it as #rrggbb'; on a host without `grim` the read fails with the reason of `grim`.

```ahk
colour := PixelGetColor(10, 20)
```

### PixelSearch

在屏幕上寻找某种颜色的位置。区域只截取一次，然后逐像素查找该颜色。它是不依赖桌面种类的检查手段。

- **参数:** &X, &Y, X1, Y1, X2, Y2, Color
- **返回:** 找到颜色时为 1，否则为 0
- **错误:** the two places are written into the variables of the caller, and they are made blank when the colour is not there; on a host without `grim` the search fails with the reason of `grim`.

```ahk
PixelSearch &x, &y, 0, 0, 100, 100, "#ff0000"
```
