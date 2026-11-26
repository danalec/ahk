# `RegExMatch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · 言語の中核: テキスト、数値、正規表現

## 説明

Where a pattern is in a text. The position counts from one and is zero when the pattern is not there.

## 構文

```ahk
RegExMatch …
```

## パラメータ

Text, Pattern, [&Output], [Start]

## 戻り値

the position of the match, or 0 when there is none

## エラー

the output must be a variable reference; the output holds an object of the class RegExMatchInfo, and a text that does not match leaves it holding the empty text.

## 例

```ahk
RegExMatch("abc123", "\d+")
```
