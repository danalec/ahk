# `LTrim`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · function · 言語の中核: テキスト、数値、正規表現

## 説明

The text without the characters at its left end.

## 構文

```ahk
S := LTrim(String, OmitChars)
```

## パラメータ

- **String** — The text.
- **OmitChars** — Default space+tab.

## 戻り値

the text without the leading characters of the set; the set is the space and the tab when the caller does not name one

## エラー

none.

## 例

```ahk
LTrim("  hello  ")
```

## 関連

[`Trim`](Trim.md), [`RTrim`](RTrim.md)
