# `FileExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 文件与目录操作

## 描述

回答名称或模式匹配到的第一个条目的属性。

## 语法

```ahk
Attrib := FileExist(Path)
```

## 参数

- **Path** — A path or a wildcard pattern.

## 返回值

以字母表示的属性：文件是 `A`，目录是 `D`，只读文件是 `RA`，没有这样的文件时是空文本。

## 错误

nothing fails: a name that matches nothing answers the empty text.

## 备注

A wildcard answers the first match in the folder's enumeration order.

## 示例

```ahk
x := FileExist("out.txt")
```

## 相关

[`DirExist`](DirExist.md), [`FileGetAttrib`](FileGetAttrib.md)
