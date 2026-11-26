# `StrPut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 語言核心：文字、數學與模式

## 描述

Writes a text into a buffer. Without a target the call answers the number of bytes the text needs, terminator included.

## 語法

```ahk
StrPut …
```

## 參數

Text, [Target], [Encoding], [Length]

## 傳回值

the number of bytes written, or the number of bytes the text needs when there is no target

## 錯誤

a target that is not a buffer is refused with "Parameter #2 of StrPut is invalid."; a text that does not fit in the size the caller names is refused with "Invalid Length.".

## 範例

```ahk
StrPut("abc", Buffer(16), "UTF-8")
```
