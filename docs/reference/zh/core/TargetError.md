# `TargetError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · class · 语言核心：文本、数学与模式

## 描述

A window/control/pixel that was not found.

## 语法

```ahk
e := TargetError(Message, What, Extra)
```

## 参数

?.

## 返回值

An error object (Message/What/Extra/File/Line/Stack).

## 示例

```ahk
throw TargetError("x")
```

## 相关

[`Error`](Error.md), [`Throw`](Throw.md), [`OnError`](OnError.md)
