# `FileSelect`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 文件与目录操作

## 描述

Shows a file-selection dialog.

## 语法

```ahk
S := FileSelect(Options, RootDir, Prompt, Filter)
```

## 参数

0..4.

## 返回值

The path(s) or empty.

## 示例

```ahk
p := FileSelect()
```

## 相关

[`DirSelect`](DirSelect.md), [`InputBox`](InputBox.md)
