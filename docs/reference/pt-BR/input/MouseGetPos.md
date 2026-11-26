# `MouseGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings e entrada

## Descrição

Lê do kernel a posição do ponteiro que esta árvore dirige. É o único ponteiro da máquina cuja posição pode ser lida: o mouse físico é um dispositivo relativo.

## Sintaxe

```ahk
MouseGetPos OutputX, OutputY, OutputWin, OutputControl, Flag
```

## Parâmetros

- **Output*** — Variables that receive the answers.

## Valor de retorno

nada; a posição é escrita nas variáveis de quem chama

## Erros

when the kernel will not answer, the desktop answers the place the runtime last put the pointer at, and says so in a note.

## Exemplo

```ahk
MouseGetPos &x, &y
```

## Relacionados

[`MouseMove`](MouseMove.md), [`PixelGetColor`](PixelGetColor.md)
