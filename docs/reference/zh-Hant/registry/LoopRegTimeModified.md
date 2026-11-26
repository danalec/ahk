# `LoopRegTimeModified`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · variable · 登錄檔 API

## 描述

The last-write time of the current registry key.

## 語法

```ahk
S := LoopRegTimeModified
```

## 參數

?.

## 傳回值

A YYYYMMDDHH24MISS timestamp.

## 範例

```ahk
t := LoopRegTimeModified
```

## 相關

[`LoopRegKey`](LoopRegKey.md), [`FormatTime`](FormatTime.md)
