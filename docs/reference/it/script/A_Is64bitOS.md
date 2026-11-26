# `A_Is64bitOS`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · variable · L'analizzatore e l'esecuzione dello script

## Descrizione

Whether the machine is a 64-bit one. The tree answers with the width of its own pointer: 1 on a 64-bit build of the runtime, 0 on a 32-bit one.

## Sintassi

```ahk
A_Is64bitOS 
```

## Parametri

nothing; it is a variable, not a call

## Valore di ritorno

1 or 0

## Errori

nothing to read; the write is refused.

## Esempio

```ahk
FileAppend A_Is64bitOS, "*"
```
