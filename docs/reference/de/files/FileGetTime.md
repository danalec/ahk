# `FileGetTime`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Datei- und Verzeichnisoperationen

## Beschreibung

Answers when a file was made, when it was changed, or when it was read.

## Syntax

```ahk
FileGetTime …
```

## Parameter

FileName, [Which]

## Rückgabewert

the stamp of v2, the fourteen digits of `YYYYMMDDHH24MISS`, in the local time of the machine.

## Fehler

ValueError when `Which` is not `M`, `C`, `A` or `CUT`; OSError when the file cannot be read.

## Beispiel

```ahk
t := FileGetTime("out.txt", "M")
```
