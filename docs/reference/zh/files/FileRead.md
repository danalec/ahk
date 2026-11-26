# `FileRead`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

读出整个文件，答案是它的文本。编码标记决定文件用什么编码读取；文件没有标记时由脚本的选项决定。命令的值要用括号来取：在表达式里写命令的方式会失败，而 v2.0.31 的参考实现也是这样失败。

## 语法

```ahk
Text := FileRead(Filename, Options)
```

## 参数

- **Filename** — The path.
- **Options** — `n (LF), `r`n, RAW, encoding (UTF-8/UTF-16/CPnnn), m<n> limit.

## 返回值

文件的文本；带 `RAW` 选项时是一个 Buffer。

## 错误

OSError when the file cannot be read; ValueError when an option is not one of v2. The options are the encoding of the file -- `UTF-8`, `UTF-16` and the code page of the machine --, `RAW` and `m<n>`, the number of bytes to read.

## 备注

A missing file throws an OSError. The encoding is detected from a BOM, else the option, else the default.

## 示例

```ahk
text := FileRead("out.txt")
```

## 相关

[`FileAppend`](FileAppend.md), [`FileOpen`](FileOpen.md), [`LoopRead`](LoopRead.md)
