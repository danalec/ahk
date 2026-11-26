# `ImageSearch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Captura de tela e funções de pixel

## Descrição

Procura uma imagem na tela. A região é capturada uma vez e a imagem é procurada nela, com a variação e a cor transparente.

## Sintaxe

```ahk
ImageSearch …
```

## Parâmetros

&X, &Y, X1, Y1, X2, Y2, Picture [, Options]

## Valor de retorno

1 quando a imagem está lá, 0 quando não está

## Erros

a picture that cannot be loaded is refused with 'Parameter #7 of ImageSearch is invalid.'; on a host without `grim` the search fails with the reason of `grim`.

## Exemplo

```ahk
ImageSearch &x, &y, 0, 0, 800, 600, "button.png"
```
