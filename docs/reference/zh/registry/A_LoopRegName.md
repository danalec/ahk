# `A_LoopRegName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 注册表 API

## 描述

注册表循环所在的注册表项名称。注册表是 Windows 的东西，而这个运行时没有：注册表循环会以“the registry is a Windows thing and this box has none.”失败，这个名字始终是空文本。

## 语法

```ahk
A_LoopRegName 
```

## 参数

nothing; it is a variable, not a call

## 返回值

空文本；这个树从不写入它

## 错误

nothing to read; the write is refused.

## 示例

```ahk
FileAppend A_LoopRegName, "*"
```
