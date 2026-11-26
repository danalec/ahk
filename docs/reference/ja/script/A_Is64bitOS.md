# `A_Is64bitOS`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · variable · パーサとスクリプトの実行

## 説明

Whether the machine is a 64-bit one. The tree answers with the width of its own pointer: 1 on a 64-bit build of the runtime, 0 on a 32-bit one.

## 構文

```ahk
A_Is64bitOS 
```

## パラメータ

nothing; it is a variable, not a call

## 戻り値

1 or 0

## エラー

nothing to read; the write is refused.

## 例

```ahk
FileAppend A_Is64bitOS, "*"
```
