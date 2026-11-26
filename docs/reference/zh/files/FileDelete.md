# `FileDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

删除一个文件，或删除某个模式匹配到的所有文件。模式没有匹配到文件不算失败。

## 语法

```ahk
FileDelete Pattern
```

## 参数

- **Pattern** — A path or wildcard.

## 返回值

无返回值

## 错误

OSError when the file cannot be taken away.

## 示例

```ahk
FileDelete "out.txt"
```

## 相关

[`DirDelete`](DirDelete.md), [`FileRecycle`](FileRecycle.md)
