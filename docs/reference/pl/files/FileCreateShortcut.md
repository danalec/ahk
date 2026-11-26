# `FileCreateShortcut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Operacje na plikach i katalogach

## Opis

Makes a shortcut to a program. On this desktop a shortcut is a `.desktop` file of the freedesktop specification, and it is made even for a target that is not there.

## Składnia

```ahk
FileCreateShortcut …
```

## Parametry

Target, LinkFile, [WorkingDir], [Args], [Description], [Icon], [IconNumber], [Run]

## Wartość zwracana

nothing

## Błędy

Error when the file of the shortcut cannot be written. The icon number and the run state are not carried.

## Przykład

```ahk
FileCreateShortcut "/bin/ls", "ls.desktop"
```
