# `IniDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

删除 ini 文件的一个键。不给键名时，整个节一起删除。

## 语法

```ahk
IniDelete …
```

## 参数

FileName, Section, [Key]

## 返回值

无返回值

## 错误

Error when the file cannot be written.

## 示例

```ahk
IniDelete "app.ini", "Main", "Name"
```
