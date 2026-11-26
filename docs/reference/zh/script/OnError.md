# `OnError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 解析器与脚本执行

## 描述

注册处理未被捕获的失败的函数。

## 语法

```ahk
OnError Function, Add
```

## 参数

- **Function** — The callback (Thrown, Mode).
- **Add** — 1/-1.

## 返回值

整数 1

## 错误

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## 示例

```ahk
OnError((err, mode) => 0)
```

## 相关

[`OnExit`](OnExit.md), [`Throw`](Throw.md)
