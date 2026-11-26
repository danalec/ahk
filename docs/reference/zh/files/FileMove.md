# `FileMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

移动一个文件，或移动某个模式匹配到的所有文件。源和目标的规则与 `FileCopy` 相同。

## 语法

```ahk
FileMove Source, Dest, Overwrite
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
FileMove "a.txt", "b.txt"
```

## 相关

[`FileCopy`](FileCopy.md), [`DirMove`](DirMove.md)
