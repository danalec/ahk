# `LoopRegKey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · variable · 注册表 API

## 描述

The name of the current subkey in a Loop Reg.

## 语法

```ahk
S := LoopRegKey
```

## 参数

?.

## 返回值

The subkey name.

## 示例

```ahk
Loop Reg, "HKCU\Software"
    s := LoopRegKey
```

## 相关

[`LoopRegName`](LoopRegName.md), [`LoopRegType`](LoopRegType.md)
