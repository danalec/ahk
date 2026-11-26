# `FileOpen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Operazioni su file e directory

## Descrizione

Opens a file and answers the object of it, of the class `File`. The modes are `r`, `w`, `a` and `rw`; the name `*` is the console of the machine.

## Sintassi

```ahk
File := FileOpen(Filename, Flags, Encoding)
```

## Parametri

- **Filename** — The path; * is the console.
- **Flags** — r/w/a/rw plus share mode.
- **Encoding** — UTF-8/UTF-16/CPnnn/RAW.

## Valore di ritorno

a `File` object.

## Errori

ValueError when the mode is not one of the four; OSError when the file cannot be opened.

## Osservazioni

The File object keeps the handle; Close releases it.

## Esempio

```ahk
f := FileOpen("out.txt", "w")
```

## Correlati

[`FileRead`](FileRead.md), [`FileAppend`](FileAppend.md)
