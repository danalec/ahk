# `SplitPath`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

把路径切成几段，并写进调用者的变量里。文件路径按反斜杠切，网址按斜杠切；网址的盘符是它的方案和主机。

## 语法

```ahk
SplitPath …
```

## 参数

Path, &OutFileName, &OutDir, &OutExt, &OutNameNoExt, &OutDrive

## 返回值

无返回值

## 错误

TypeError when a piece is not a variable to write into.

## 示例

```ahk
SplitPath "C:\dir\file.txt", &name, &dir, &ext
```
