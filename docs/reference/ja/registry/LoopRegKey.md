# `LoopRegKey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · variable · レジストリの API

## 説明

The name of the current subkey in a Loop Reg.

## 構文

```ahk
S := LoopRegKey
```

## パラメータ

?.

## 戻り値

The subkey name.

## 例

```ahk
Loop Reg, "HKCU\Software"
    s := LoopRegKey
```

## 関連

[`LoopRegName`](LoopRegName.md), [`LoopRegType`](LoopRegType.md)
