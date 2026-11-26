# `A_IsAdmin`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · variable · 解析器與腳本執行

## 描述

Whether the script runs with the rights of an administrator: the effective user id, and the root is the one that is zero.

## 語法

```ahk
A_IsAdmin 
```

## 參數

nothing; it is a variable, not a call

## 傳回值

1 when the script is elevated, 0 when it is not

## 錯誤

the read fails when the machine cannot say; the write is refused.

## 範例

```ahk
FileAppend A_IsAdmin, "*"
```
