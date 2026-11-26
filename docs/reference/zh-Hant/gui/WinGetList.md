# `WinGetList`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · GUI 視窗與視窗自動化

## 描述

The identifiers of every window that matches the title that was given.

## 語法

```ahk
WinGetList …
```

## 參數

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## 傳回值

an array of the identifiers, empty when nothing matches

## 錯誤

the text of the window and the exclusions are not carried yet; a question the compositor cannot answer fails with its reason.

## 範例

```ahk
list := WinGetList("Untitled")
```
