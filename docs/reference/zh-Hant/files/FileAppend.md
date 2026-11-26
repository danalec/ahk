# `FileAppend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 檔案與目錄操作

## 描述

Adds text at the end of a file. The name of the file may be `*`, and the text then goes to the output of the report; inside the body of a `Loop Read` the name may be left out, and the file of the loop is the one that is written.

## 語法

```ahk
FileAppend Text, Filename, Options
```

## 參數

- **Text** — The text or Buffer.
- **Filename** — The path; * is stdout.
- **Options** — Encoding, `n, RAW.

## 傳回值

nothing

## 錯誤

ValueError when the name of the file is empty or when an option is not one of v2; OSError when the file cannot be opened for writing. The options are the encoding of the file -- `UTF-8`, `UTF-16` and the code page of the machine --, `RAW` and a linefeed; a code page the tree does not carry is refused with the reason.

## 備註

Writing to * goes to the report output on this tree. A Buffer writes its bytes.

## 範例

```ahk
FileAppend "hello`n", "out.txt"
```

## 相關

[`FileRead`](FileRead.md), [`FileOpen`](FileOpen.md)
