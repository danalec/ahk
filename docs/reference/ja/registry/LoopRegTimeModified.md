# `LoopRegTimeModified`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · variable · レジストリの API

## 説明

The last-write time of the current registry key.

## 構文

```ahk
S := LoopRegTimeModified
```

## パラメータ

?.

## 戻り値

A YYYYMMDDHH24MISS timestamp.

## 例

```ahk
t := LoopRegTimeModified
```

## 関連

[`LoopRegKey`](LoopRegKey.md), [`FormatTime`](FormatTime.md)
