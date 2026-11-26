# `Reload`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 解析器与脚本执行

## 描述

在一个自己的进程里重新启动脚本：旧的实例结束，新的实例带着同样的文件和同样的参数继续运行。来自标准输入的脚本没有文件，此时 v2 什么也不做。

## 语法

```ahk
Reload
```

## 参数

none

## 返回值

空文本

## 错误

none; the name takes no parameter, and a script without a file is not refused -- the call simply does nothing.

## 备注

The new instance replaces the old; single-instance rules apply.

## 示例

```ahk
Reload
```

## 相关

[`ExitApp`](ExitApp.md), [`#SingleInstance`](#SingleInstance.md)
