# `StrLen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · 言語の中核: テキスト、数値、正規表現

## 説明

Counts the characters of a text in the code units of UTF-16, so a character outside the basic plane counts as two.

## 構文

```ahk
Count := StrLen(String)
```

## パラメータ

- **String** — The text.

## 戻り値

the number of code units

## エラー

none; a value that is not text is read as its text.

## 備考

Astral characters count as one each on this tree.

## 例

```ahk
StrLen("hello")
```

## 関連

[`SubStr`](SubStr.md), [`InStr`](InStr.md)
