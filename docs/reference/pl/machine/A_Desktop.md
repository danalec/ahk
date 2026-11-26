# `A_Desktop`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · variable · Maszyna, procesy i środowisko

## Opis

The folder of the desktop of the user: the folder the machine knows, and `$HOME/Desktop` when it knows none.

## Składnia

```ahk
A_Desktop 
```

## Parametry

nothing; it is a variable, not a call

## Wartość zwracana

the path of the folder, as a text

## Błędy

the read fails with "the machine has no such folder." when the machine has no such folder; the write is refused.

## Przykład

```ahk
FileAppend A_Desktop, "*"
```
