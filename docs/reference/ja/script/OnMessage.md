# `OnMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · パーサとスクリプトの実行

## 説明

Registers a function to handle a window message.

## 構文

```ahk
OnMessage MsgNumber, Function, MaxThreads
```

## パラメータ

1..3.

## 戻り値

Nothing.

## 例

```ahk
OnMessage(0x100, Fn)
```

## 関連

[`OnExit`](OnExit.md), [`OnError`](OnError.md), [`SendMessage`](SendMessage.md)
