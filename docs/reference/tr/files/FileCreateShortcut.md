# `FileCreateShortcut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Dosya ve dizin işlemleri

## Açıklama

Makes a shortcut to a program. On this desktop a shortcut is a `.desktop` file of the freedesktop specification, and it is made even for a target that is not there.

## Sözdizimi

```ahk
FileCreateShortcut …
```

## Parametreler

Target, LinkFile, [WorkingDir], [Args], [Description], [Icon], [IconNumber], [Run]

## Dönüş değeri

nothing

## Hatalar

Error when the file of the shortcut cannot be written. The icon number and the run state are not carried.

## Örnek

```ahk
FileCreateShortcut "/bin/ls", "ls.desktop"
```
