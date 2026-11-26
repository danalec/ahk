# `Ceil`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · 言語の中核: テキスト、数値、正規表現

## 説明

The whole number above a number.

## 構文

```ahk
N := Ceil(Number)
```

## パラメータ

- **Number** — The value.

## 戻り値

the whole number, a float when the number was not whole

## エラー

a value that is not a number is refused.

## 例

```ahk
Ceil(1.2)
```

## 関連

[`Floor`](Floor.md), [`Round`](Round.md)
