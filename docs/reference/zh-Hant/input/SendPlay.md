# `SendPlay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 熱鍵、熱字串與輸入

## 描述

Refused. The play method of v2 does not send keystrokes: it posts the messages to the active window, and there is no window here to post them to.

## 語法

```ahk
SendPlay …
```

## 參數

Keys

## 傳回值

nothing; the statement fails

## 錯誤

the play method of v2 posts the messages to the active window, and there is no window here to post them to.

## 範例

```ahk
SendPlay "{Enter}"
```
