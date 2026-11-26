# `Click`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings e entrada

## Descrição

Clica com o ponteiro do backend, que é um dispositivo uinput absoluto: move até o lugar, assenta, pressiona e solta.

## Sintaxe

```ahk
Click Options
```

## Parâmetros

- **Options** — X Y, button, count, Down/Up, Rel.

## Valor de retorno

nada

## Erros

the wheel, the fourth and the fifth buttons, the pressing of a button and the place relative to the pointer are refused as not carried yet; on a host without `grim` the click fails with the reason of the screen, because the pointer needs the size of the screen.

## Observações

On Linux the absolute uinput pointer / XTEST.

## Exemplo

```ahk
Click 100, 200
```

## Relacionados

[`MouseMove`](MouseMove.md), [`MouseClick`](MouseClick.md), [`Send`](Send.md)
