# `A_TickCount`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · variable · 機器、行程與環境

## 描述

The number of milliseconds the runtime has counted since an arbitrary origin. The count never goes backwards.

## 語法

```ahk
A_TickCount 
```

## 參數

nothing; it is a variable, not a call

## 傳回值

the number of milliseconds, as a number

## 錯誤

the write is refused; the name is a built-in variable that the dialect does not let a script write.

## 範例

```ahk
FileAppend A_TickCount, "*"
```
