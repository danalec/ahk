# `A_SendMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · variable · 熱鍵、熱字串與輸入

## 描述

The method that a `Send` uses: `Input`, `Event`, `Play` or `InputThenPlay`.

## 語法

```ahk
A_SendMode 
```

## 參數

nothing; it is a variable, not a call

## 傳回值

the name of the mode, as a text

## 錯誤

the write takes the name of one of the four modes, and any other name is refused with "Invalid value.".

## 範例

```ahk
FileAppend A_SendMode, "*"
```
