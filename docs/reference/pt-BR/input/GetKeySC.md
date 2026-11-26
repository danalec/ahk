# `GetKeySC`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings e entrada

## Descrição

O código do evento de entrada Linux da tecla. É o código que o backend escreve no seu próprio dispositivo, e não o scan code do teclado de PC: as duas numerações divergem nas teclas estendidas.

## Sintaxe

```ahk
GetKeySC …
```

## Parâmetros

KeyName

## Valor de retorno

o código de evento de entrada, um número

## Erros

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## Exemplo

```ahk
sc := GetKeySC("Enter")
```
