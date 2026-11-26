# `Ord`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · 言語の中核: テキスト、数値、正規表現

## 説明

The code point of the first character of a text.

## 構文

```ahk
N := Ord(String)
```

## パラメータ

- **String** — The text.

## 戻り値

the code point, or zero when the text has no character

## エラー

none; a lone surrogate answers with the code unit of it.

## 例

```ahk
Ord("A")
```

## 関連

[`Chr`](Chr.md), [`Asc`](Asc.md)
