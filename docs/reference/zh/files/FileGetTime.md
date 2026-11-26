# `FileGetTime`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

回答文件是什么时候创建的、什么时候被修改的、什么时候被读取的。

## 语法

```ahk
FileGetTime …
```

## 参数

FileName, [Which]

## 返回值

v2 的时间戳，`YYYYMMDDHH24MISS` 的十四位数字，用机器的本地时间。

## 错误

ValueError when `Which` is not `M`, `C`, `A` or `CUT`; OSError when the file cannot be read.

## 示例

```ahk
t := FileGetTime("out.txt", "M")
```
