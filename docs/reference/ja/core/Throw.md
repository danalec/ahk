# `Throw`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*KeySharp* · function · 言語の中核: テキスト、数値、正規表現

## 説明

A statement that raises a value. The value is usually an error object, and it may be any value.

## 構文

```ahk
Throw 
```

## パラメータ

Value

## 戻り値

nothing; the statement leaves the flow of the script

## エラー

none; the value is caught by the catch that names its class.

## 例

```ahk
throw ValueError("boom")
```
