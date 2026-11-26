# `FileCreateShortcut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Operazioni su file e directory

## Descrizione

Makes a shortcut to a program. On this desktop a shortcut is a `.desktop` file of the freedesktop specification, and it is made even for a target that is not there.

## Sintassi

```ahk
FileCreateShortcut …
```

## Parametri

Target, LinkFile, [WorkingDir], [Args], [Description], [Icon], [IconNumber], [Run]

## Valore di ritorno

nothing

## Errori

Error when the file of the shortcut cannot be written. The icon number and the run state are not carried.

## Esempio

```ahk
FileCreateShortcut "/bin/ls", "ls.desktop"
```
