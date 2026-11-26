# `SendText`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 熱鍵、熱字串與輸入

## 描述

Types the text as it stands: the braces of v2 are not read here, and the text goes to the keyboard one character at a time.

## 語法

```ahk
SendText …
```

## 參數

Text

## 傳回值

nothing

## 錯誤

the keyboard of the backend is a Linux uinput device, so on a host that is not Linux the text is not typed.

## 範例

```ahk
SendText "a literal {braced} text"
```
