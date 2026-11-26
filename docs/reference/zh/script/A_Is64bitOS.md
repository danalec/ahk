# `A_Is64bitOS`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 解析器与脚本执行

## 描述

机器是否是 64 位。这个树用它自身指针的宽度回答：运行时的 64 位构建是 1，32 位构建是 0。

## 语法

```ahk
A_Is64bitOS 
```

## 参数

nothing; it is a variable, not a call

## 返回值

1 或 0

## 错误

nothing to read; the write is refused.

## 示例

```ahk
FileAppend A_Is64bitOS, "*"
```
