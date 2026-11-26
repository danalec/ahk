# `A_LoopField`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · variable · 檔案與目錄操作

## 描述

The field the parse loop is on: the piece of the text between two delimiters.

## 語法

```ahk
A_LoopField 
```

## 參數

nothing; it is a variable, not a call

## 傳回值

the field, as a text; the empty text before the first parse loop

## 錯誤

nothing to read; the write is refused.

## 範例

```ahk
Loop Parse "a,b,c", ","
    FileAppend A_LoopField, "*"
```
