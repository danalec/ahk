# `A_ThisHotkey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · variable · Tasti rapidi, stringhe di attivazione e input

## Descrizione

The name of the hotkey that runs. The hotkeys are the ones of the hotkey system of the runtime, and the name is the empty text when no hotkey runs.

## Sintassi

```ahk
A_ThisHotkey 
```

## Parametri

nothing; it is a variable, not a call

## Valore di ritorno

the name of the hotkey, as a text; the empty text when no hotkey runs

## Errori

nothing to read; the write is refused.

## Esempio

```ahk
FileAppend A_ThisHotkey, "*"
```
