# `A_KeyDelay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · variable · 熱鍵、熱字串與輸入

## 描述

The delay between the keystrokes of a `Send`, in milliseconds. It is a setting of the run, and it is ten by default.

## 語法

```ahk
A_KeyDelay 
```

## 參數

nothing; it is a variable, not a call

## 傳回值

the delay, as a number

## 錯誤

the write takes the number of the value, and a text that holds no number is zero.

## 範例

```ahk
FileAppend A_KeyDelay, "*"
```
