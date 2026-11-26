# `FileCopy`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

复制一个文件，或复制某个模式匹配到的所有文件。目标以分隔符结尾、或者目标本身是目录时，文件被复制进那个目录。

## 语法

```ahk
FileCopy Source, Dest, Overwrite
```

## 参数

- **Source** — Path/wildcard.
- **Dest** — Destination.
- **Overwrite** — 1 to overwrite.

## 返回值

无返回值

## 错误

Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

## 示例

```ahk
FileCopy "a.txt", "b.txt"
```

## 相关

[`FileMove`](FileMove.md), [`DirCopy`](DirCopy.md)
