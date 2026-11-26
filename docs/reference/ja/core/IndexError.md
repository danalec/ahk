# `IndexError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · class · 言語の中核: テキスト、数値、正規表現

## 説明

An index out of the bounds of a collection.

## 構文

```ahk
e := IndexError(Message, What, Extra)
```

## パラメータ

?.

## 戻り値

An error object (Message/What/Extra/File/Line/Stack).

## 例

```ahk
throw IndexError("x")
```

## 関連

[`Error`](Error.md), [`Throw`](Throw.md), [`OnError`](OnError.md)
