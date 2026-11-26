# `A_ComputerName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 机器、进程与环境

## 描述

计算机的名称，按网络看到的：先是内核，内核没有名字时用环境中的 `HOSTNAME`。

## 语法

```ahk
A_ComputerName 
```

## 参数

nothing; it is a variable, not a call

## 返回值

名称，作为文本

## 错误

the read fails when neither the kernel nor the session carries the name; the write is refused.

## 示例

```ahk
FileAppend A_ComputerName, "*"
```
