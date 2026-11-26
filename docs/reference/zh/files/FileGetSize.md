# `FileGetSize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

回答一个文件有多大。

## 语法

```ahk
FileGetSize …
```

## 参数

FileName, [Unit]

## 返回值

大小，向下取整；目录是零。

## 错误

ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

## 示例

```ahk
n := FileGetSize("out.txt", "K")
```
