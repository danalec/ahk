# `DirDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 文件与目录操作

## 描述

删除一个目录。目录非空时，除非要求递归，否则目录会保留下来。

## 语法

```ahk
DirDelete Path, Recurse
```

## 参数

- **Path** — The directory.
- **Recurse** — 1 to delete contents.

## 返回值

无返回值；命令不回答任何东西。

## 错误

Error `Failed` when the directory is not there, or when it is not empty and the recursion is not asked for.

## 示例

```ahk
d := DirDelete("out", 1)
```

## 相关

[`DirCreate`](DirCreate.md), [`FileDelete`](FileDelete.md)
