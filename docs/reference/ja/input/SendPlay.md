# `SendPlay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · ホットキー、ホット文字列、入力

## 説明

Refused. The play method of v2 does not send keystrokes: it posts the messages to the active window, and there is no window here to post them to.

## 構文

```ahk
SendPlay …
```

## パラメータ

Keys

## 戻り値

nothing; the statement fails

## エラー

the play method of v2 posts the messages to the active window, and there is no window here to post them to.

## 例

```ahk
SendPlay "{Enter}"
```
