# `OnError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · パーサとスクリプトの実行

## 説明

Registers the function that handles a failure the script did not catch.

## 構文

```ahk
OnError Function, Add
```

## パラメータ

- **Function** — The callback (Thrown, Mode).
- **Add** — 1/-1.

## 戻り値

the integer 1

## エラー

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## 例

```ahk
OnError((err, mode) => 0)
```

## 関連

[`OnExit`](OnExit.md), [`Throw`](Throw.md)
