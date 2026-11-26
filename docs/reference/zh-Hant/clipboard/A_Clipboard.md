# `A_Clipboard`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · variable · 剪貼簿

## 描述

The text of the clipboard of the desktop. A script may write it, and the concatenation adds to what is there.

## 語法

```ahk
A_Clipboard 
```

## 參數

nothing; it is a variable, not a call

## 傳回值

the text of the clipboard

## 錯誤

the read fails with the reason of the desktop; the write of a number is refused, because the clipboard holds a text.

## 範例

```ahk
A_Clipboard := "hello"
FileAppend A_Clipboard, "*"
```
