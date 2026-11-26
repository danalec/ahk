# `PixelGetColor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Captura de tela e funções de pixel

## Descrição

A cor de um pixel, lida do quadro que o compositor desenha. O backend captura uma região 1x1 com `grim` e lê o pixel.

## Sintaxe

```ahk
C := PixelGetColor(X, Y, Mode)
```

## Parâmetros

- **X/Y** — Screen coords.
- **Mode** — Alt/Slow/RGB.

## Valor de retorno

a cor, como `#rrggbb`

## Erros

a colour written as a number is refused with 'is not a colour: write it as #rrggbb'; on a host without `grim` the read fails with the reason of `grim`.

## Exemplo

```ahk
colour := PixelGetColor(10, 20)
```

## Relacionados

[`PixelSearch`](PixelSearch.md), [`ImageSearch`](ImageSearch.md)
