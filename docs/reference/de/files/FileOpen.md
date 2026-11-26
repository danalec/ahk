# `FileOpen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · Datei- und Verzeichnisoperationen

## Beschreibung

Opens a file and answers the object of it, of the class `File`. The modes are `r`, `w`, `a` and `rw`; the name `*` is the console of the machine.

## Syntax

```ahk
File := FileOpen(Filename, Flags, Encoding)
```

## Parameter

- **Filename** — The path; * is the console.
- **Flags** — r/w/a/rw plus share mode.
- **Encoding** — UTF-8/UTF-16/CPnnn/RAW.

## Rückgabewert

a `File` object.

## Fehler

ValueError when the mode is not one of the four; OSError when the file cannot be opened.

## Bemerkungen

The File object keeps the handle; Close releases it.

## Beispiel

```ahk
f := FileOpen("out.txt", "w")
```

## Verwandt

[`FileRead`](FileRead.md), [`FileAppend`](FileAppend.md)
