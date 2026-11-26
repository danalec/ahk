# `A_Desktop`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 机器、进程与环境

## 描述

用户桌面的文件夹：机器知道的文件夹；机器不知道时是 `$HOME/Desktop`。

## 语法

```ahk
A_Desktop 
```

## 参数

nothing; it is a variable, not a call

## 返回值

文件夹的路径，作为文本

## 错误

the read fails with "the machine has no such folder." when the machine has no such folder; the write is refused.

## 示例

```ahk
FileAppend A_Desktop, "*"
```
