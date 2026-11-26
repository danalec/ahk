# `FileOpen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 檔案與目錄操作

## 描述

Opens a file and answers the object of it, of the class `File`. The modes are `r`, `w`, `a` and `rw`; the name `*` is the console of the machine.

## 語法

```ahk
File := FileOpen(Filename, Flags, Encoding)
```

## 參數

- **Filename** — The path; * is the console.
- **Flags** — r/w/a/rw plus share mode.
- **Encoding** — UTF-8/UTF-16/CPnnn/RAW.

## 傳回值

a `File` object.

## 錯誤

ValueError when the mode is not one of the four; OSError when the file cannot be opened.

## 備註

The File object keeps the handle; Close releases it.

## 範例

```ahk
f := FileOpen("out.txt", "w")
```

## 相關

[`FileRead`](FileRead.md), [`FileAppend`](FileAppend.md)
