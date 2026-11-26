# `DirCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 文件与目录操作

## 描述

创建目录，途中缺少的目录也一并创建。

## 语法

```ahk
DirCreate Path
```

## 参数

- **Path** — The directory.

## 返回值

无返回值

## 错误

OSError when the directory cannot be made.

## 示例

```ahk
DirCreate "out"
```

## 相关

[`DirDelete`](DirDelete.md), [`DirCopy`](DirCopy.md)
