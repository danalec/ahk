# `File`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · method · `Error` · 文件与目录操作

## 描述

机器上已打开文件的类。这个类造不出来：`File(...)` 回答“Invalid base.”；文件由内置函数 `FileOpen` 造出，这是唯一的办法。

## 语法

```ahk
File 
```

## 参数

nothing that makes it; the arguments of `FileOpen` open the file

## 返回值

类为 `File` 的对象

## 错误

the call of the class is refused with "Invalid base.".

## 示例

```ahk
f := FileOpen("notes.txt", "w")
f.Close()
```
