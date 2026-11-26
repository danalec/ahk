# `FileGetVersion`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

The version of a file.

## 语法

```ahk
S := FileGetVersion(Filename)
```

## 参数

1..255.

## 返回值

The version.

## 示例

```ahk
v := FileGetVersion("a.exe")
```

## 相关

[`FileExist`](FileExist.md)
