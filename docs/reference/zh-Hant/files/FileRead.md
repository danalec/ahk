# `FileRead`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 檔案與目錄操作

## 描述

Reads a whole file and answers its text. The mark of the encoding decides the encoding of the file, and the option of the script decides when the file has no mark. The value of the command is taken with the parenthesis, the way `FileRead("out.txt")` is written: the command style inside an expression fails with "Expected a String but got a Func.", and the reference of v2.0.31 fails the same way.

## 語法

```ahk
Text := FileRead(Filename, Options)
```

## 參數

- **Filename** — The path.
- **Options** — `n (LF), `r`n, RAW, encoding (UTF-8/UTF-16/CPnnn), m<n> limit.

## 傳回值

the text of the file; a Buffer when the option `RAW` is there.

## 錯誤

OSError when the file cannot be read; ValueError when an option is not one of v2. The options are the encoding of the file -- `UTF-8`, `UTF-16` and the code page of the machine --, `RAW` and `m<n>`, the number of bytes to read.

## 備註

A missing file throws an OSError. The encoding is detected from a BOM, else the option, else the default.

## 範例

```ahk
text := FileRead("out.txt")
```

## 相關

[`FileAppend`](FileAppend.md), [`FileOpen`](FileOpen.md), [`LoopRead`](LoopRead.md)
