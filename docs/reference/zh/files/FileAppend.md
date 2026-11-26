# `FileAppend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

把文本追加到文件末尾。文件名可以是 `*`，此时文本进入报告的输出；在 `Loop Read` 的循环体内可以省略文件名，写入的是循环自己的文件。

## 语法

```ahk
FileAppend Text, Filename, Options
```

## 参数

- **Text** — The text or Buffer.
- **Filename** — The path; * is stdout.
- **Options** — Encoding, `n, RAW.

## 返回值

无返回值

## 错误

ValueError when the name of the file is empty or when an option is not one of v2; OSError when the file cannot be opened for writing. The options are the encoding of the file -- `UTF-8`, `UTF-16` and the code page of the machine --, `RAW` and a linefeed; a code page the tree does not carry is refused with the reason.

## 备注

Writing to * goes to the report output on this tree. A Buffer writes its bytes.

## 示例

```ahk
FileAppend "hello`n", "out.txt"
```

## 相关

[`FileRead`](FileRead.md), [`FileOpen`](FileOpen.md)
