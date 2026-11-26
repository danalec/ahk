# `PixelSearch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Captura de tela e funções de pixel

## Descrição

Onde uma cor está na tela. A região é capturada uma vez e percorrida atrás do primeiro pixel daquela cor. É a checagem que funciona em qualquer desktop.

## Sintaxe

```ahk
PixelSearch OutputX, OutputY, X1, Y1, X2, Y2, Color, Variation
```

## Parâmetros

- **Output*** — Receive the found coords.
- **Region** — The rectangle.
- **Color** — 0xRRGGBB.
- **Variation** — 0..255 per channel.

## Valor de retorno

1 quando a cor está lá, 0 quando não está

## Erros

the two places are written into the variables of the caller, and they are made blank when the colour is not there; on a host without `grim` the search fails with the reason of `grim`.

## Exemplo

```ahk
PixelSearch &x, &y, 0, 0, 100, 100, "#ff0000"
```

## Relacionados

[`PixelGetColor`](PixelGetColor.md), [`ImageSearch`](ImageSearch.md)
