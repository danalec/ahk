# `LoopRegKey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · variable · 登錄檔 API

## 描述

The name of the current subkey in a Loop Reg.

## 語法

```ahk
S := LoopRegKey
```

## 參數

?.

## 傳回值

The subkey name.

## 範例

```ahk
Loop Reg, "HKCU\Software"
    s := LoopRegKey
```

## 相關

[`LoopRegName`](LoopRegName.md), [`LoopRegType`](LoopRegType.md)
