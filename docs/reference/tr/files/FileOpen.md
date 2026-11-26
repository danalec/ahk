# `FileOpen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Dosya ve dizin işlemleri

## Açıklama

Opens a file and answers the object of it, of the class `File`. The modes are `r`, `w`, `a` and `rw`; the name `*` is the console of the machine.

## Sözdizimi

```ahk
File := FileOpen(Filename, Flags, Encoding)
```

## Parametreler

- **Filename** — The path; * is the console.
- **Flags** — r/w/a/rw plus share mode.
- **Encoding** — UTF-8/UTF-16/CPnnn/RAW.

## Dönüş değeri

a `File` object.

## Hatalar

ValueError when the mode is not one of the four; OSError when the file cannot be opened.

## Notlar

The File object keeps the handle; Close releases it.

## Örnek

```ahk
f := FileOpen("out.txt", "w")
```

## İlgili

[`FileRead`](FileRead.md), [`FileAppend`](FileAppend.md)
