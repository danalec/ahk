# `Chr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · 言語の中核: テキスト、数値、正規表現

## 説明

The character with a code point.

## 構文

```ahk
S := Chr(Number)
```

## パラメータ

- **Number** — The code point.

## 戻り値

the text of that character

## エラー

a code outside 0 to 0x10FFFF is an invalid parameter; a surrogate code is a text of its own.

## 例

```ahk
Chr(65)
```

## 関連

[`Ord`](Ord.md), [`Asc`](Asc.md)
