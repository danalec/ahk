# `A_OSVersion`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 解析器与脚本执行

## 描述

机器操作系统的版本：内核的发行版，从 `/proc/sys/kernel/osrelease` 读出。

## 语法

```ahk
A_OSVersion 
```

## 参数

nothing; it is a variable, not a call

## 返回值

版本，作为文本

## 错误

the read fails when the release of the kernel cannot be read; the write is refused.

## 示例

```ahk
FileAppend A_OSVersion, "*"
```
