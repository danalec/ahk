# `A_Desktop`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · variable · 機器、行程與環境

## 描述

The folder of the desktop of the user: the folder the machine knows, and `$HOME/Desktop` when it knows none.

## 語法

```ahk
A_Desktop 
```

## 參數

nothing; it is a variable, not a call

## 傳回值

the path of the folder, as a text

## 錯誤

the read fails with "the machine has no such folder." when the machine has no such folder; the write is refused.

## 範例

```ahk
FileAppend A_Desktop, "*"
```
