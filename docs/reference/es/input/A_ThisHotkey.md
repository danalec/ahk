# `A_ThisHotkey`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · variable · Hotkeys, hotstrings y entrada

## Descripción

The name of the hotkey that runs. The hotkeys are the ones of the hotkey system of the runtime, and the name is the empty text when no hotkey runs.

## Sintaxis

```ahk
A_ThisHotkey 
```

## Parámetros

nothing; it is a variable, not a call

## Valor de retorno

the name of the hotkey, as a text; the empty text when no hotkey runs

## Errores

nothing to read; the write is refused.

## Ejemplo

```ahk
FileAppend A_ThisHotkey, "*"
```
