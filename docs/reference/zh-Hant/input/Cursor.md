# `Cursor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · variable · 熱鍵、熱字串與輸入

## 描述

The shape of the cursor of the pointer, the name the v2 gives it. The shape belongs to the machine, and the machine is asked for it.

## 語法

```ahk
Cursor 
```

## 參數

nothing; it is a variable, not a call

## 傳回值

the name of the shape, as a text ("Arrow", "IBeam", "Wait", ...)

## 錯誤

the read fails with the reason of the desktop when the desktop cannot tell the shape of the cursor; the write is refused.

## 範例

```ahk
FileAppend A_Cursor, "*"
```
