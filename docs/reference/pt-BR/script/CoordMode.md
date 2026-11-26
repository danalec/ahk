# `CoordMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · O analisador e a execução do script

## Descrição

Diz como as coordenadas são lidas. Os cinco alvos de v2 são o mouse, os pixels, o cursor de texto, o menu e o tooltip; a árvore não tem janelas, então os modos de cursor, menu e tooltip são guardados do mesmo jeito e nada os usa.

## Sintaxe

```ahk
CoordMode …
```

## Parâmetros

Target [, Relative]

## Valor de retorno

o modo anterior

## Erros

a target or a relative that v2 does not know is refused as an invalid parameter.

## Exemplo

```ahk
CoordMode "Mouse", "Screen"
```
