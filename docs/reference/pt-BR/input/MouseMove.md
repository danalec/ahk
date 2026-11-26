# `MouseMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings e entrada

## Descrição

Move o ponteiro do backend até um lugar da tela. O backend move o ponteiro até o lugar, e não a partir de onde ele está.

## Sintaxe

```ahk
MouseMove X, Y, Speed
```

## Parâmetros

- **X/Y** — Screen coords (or relative with R).
- **Speed** — 0..100.

## Valor de retorno

nada

## Erros

the relative place of the move and the speed of the move are refused as not carried yet.

## Exemplo

```ahk
MouseMove 400, 300
```

## Relacionados

[`Click`](Click.md), [`MouseGetPos`](MouseGetPos.md)
