# `IsNumber`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · function · 言語の中核: テキスト、数値、正規表現

## 説明

Whether a value is a number: an integer, a float, or a text that holds one. A hexadecimal text is a number too.

## 構文

```ahk
B := IsNumber(Value)
```

## パラメータ

- **Value** — Any value.

## 戻り値

1 when it is a number, 0 when it is not

## エラー

a second parameter is refused with "the code page of the text is not carried yet.".

## 例

```ahk
IsNumber("42")
```

## 関連

[`IsInteger`](IsInteger.md), [`IsFloat`](IsFloat.md)
