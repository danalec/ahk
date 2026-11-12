# Captura de tela e funções de pixel

| [English](../index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Idioma: Português (Brasil)

Os nomes da família **Captura de tela e funções de pixel** nos três dialetos, com o que esta árvore responde por cada um. A família é a área em que um script pensa; os nomes são a totalidade dela.

| Nome | Dialeto | Tipo | Classe | Parâmetros | Estado | Sonda |
|---|---|---|---|---|---|---|
| `A_ScreenDPI` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `A_ScreenHeight` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `A_ScreenWidth` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `ImageSearch` | AutoHotkey v2.0.19 | command | -- | 7 | valor | -- |
| `ImageSearch` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | valor | -- |
| `LoadPicture` | AutoHotkey v2.0.19 | command | -- | 1 to 3 | valor | -- |
| `LoadPicture` | AutoHotkey v1.1.37.02 | function | -- | 1 to 3 | valor | -- |
| `PixelGetColor` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | valor | -- |
| `PixelGetColor` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | valor | -- |
| `PixelSearch` | AutoHotkey v2.0.19 | command | -- | 7 to 8 | valor | -- |
| `PixelSearch` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | valor | -- |
| `ScreenDPI` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `ScreenHeight` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `ScreenWidth` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |

## O que a arvore faz com ele

### A_ScreenHeight

A altura da tela da área de trabalho, em pixels.

- **Parâmetros:** nothing; it is a variable, not a call
- **Responde:** a altura, como um número
- **Erros:** the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

```ahk
FileAppend A_ScreenHeight, "*"
```

### A_ScreenWidth

A largura da tela da área de trabalho, em pixels.

- **Parâmetros:** nothing; it is a variable, not a call
- **Responde:** a largura, como um número
- **Erros:** the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

```ahk
FileAppend A_ScreenWidth, "*"
```

### ImageSearch

Procura uma imagem na tela. A região é capturada uma vez e a imagem é procurada nela, com a variação e a cor transparente.

- **Parâmetros:** &X, &Y, X1, Y1, X2, Y2, Picture [, Options]
- **Responde:** 1 quando a imagem está lá, 0 quando não está
- **Erros:** a picture that cannot be loaded is refused with 'Parameter #7 of ImageSearch is invalid.'; on a host without `grim` the search fails with the reason of `grim`.

```ahk
ImageSearch &x, &y, 0, 0, 800, 600, "button.png"
```

### ImageSearch

Procura uma imagem na tela. A região é capturada uma vez e a imagem é procurada nela, com a variação e a cor transparente.

- **Parâmetros:** &X, &Y, X1, Y1, X2, Y2, Picture [, Options]
- **Responde:** 1 quando a imagem está lá, 0 quando não está
- **Erros:** a picture that cannot be loaded is refused with 'Parameter #7 of ImageSearch is invalid.'; on a host without `grim` the search fails with the reason of `grim`.

```ahk
ImageSearch &x, &y, 0, 0, 800, 600, "button.png"
```

### PixelGetColor

A cor de um pixel, lida do quadro que o compositor desenha. O backend captura uma região 1x1 com `grim` e lê o pixel.

- **Parâmetros:** X, Y [, Default]
- **Responde:** a cor, como `#rrggbb`
- **Erros:** a colour written as a number is refused with 'is not a colour: write it as #rrggbb'; on a host without `grim` the read fails with the reason of `grim`.

```ahk
colour := PixelGetColor(10, 20)
```

### PixelGetColor

A cor de um pixel, lida do quadro que o compositor desenha. O backend captura uma região 1x1 com `grim` e lê o pixel.

- **Parâmetros:** X, Y [, Default]
- **Responde:** a cor, como `#rrggbb`
- **Erros:** a colour written as a number is refused with 'is not a colour: write it as #rrggbb'; on a host without `grim` the read fails with the reason of `grim`.

```ahk
colour := PixelGetColor(10, 20)
```

### PixelSearch

Onde uma cor está na tela. A região é capturada uma vez e percorrida atrás do primeiro pixel daquela cor. É a checagem que funciona em qualquer desktop.

- **Parâmetros:** &X, &Y, X1, Y1, X2, Y2, Color
- **Responde:** 1 quando a cor está lá, 0 quando não está
- **Erros:** the two places are written into the variables of the caller, and they are made blank when the colour is not there; on a host without `grim` the search fails with the reason of `grim`.

```ahk
PixelSearch &x, &y, 0, 0, 100, 100, "#ff0000"
```

### PixelSearch

Onde uma cor está na tela. A região é capturada uma vez e percorrida atrás do primeiro pixel daquela cor. É a checagem que funciona em qualquer desktop.

- **Parâmetros:** &X, &Y, X1, Y1, X2, Y2, Color
- **Responde:** 1 quando a cor está lá, 0 quando não está
- **Erros:** the two places are written into the variables of the caller, and they are made blank when the colour is not there; on a host without `grim` the search fails with the reason of `grim`.

```ahk
PixelSearch &x, &y, 0, 0, 100, 100, "#ff0000"
```
