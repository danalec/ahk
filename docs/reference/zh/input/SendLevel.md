# `SendLevel`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 热键、热字符串与输入

## 描述

保存脚本所发送事件的级别，并返回之前的级别。该级别用于告诉热键忽略哪些模拟事件，目前还没有东西读取它。

## 语法

```ahk
SendLevel …
```

## 参数

Level

## 返回值

之前的级别，一个数字

## 错误

a level outside 0 to 100 is refused as an invalid parameter.

## 示例

```ahk
SendLevel 1
```
