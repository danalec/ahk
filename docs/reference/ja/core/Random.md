# `Random`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · 言語の中核: テキスト、数値、正規表現

## 説明

A number between the two the caller names. Without arguments it is a number between zero and one; with one it is a whole number below it; with two it is a whole number between them, both ends counted.

## 構文

```ahk
N := Random(Min, Max)
```

## パラメータ

- **Min/Max** — The range; omitted = float 0..1.
- **Seed** — With a single arg, reseeds.

## 戻り値

the number

## エラー

none.

## 例

```ahk
Random(1, 6)
```

## 関連

[`Sort`](Sort.md)
