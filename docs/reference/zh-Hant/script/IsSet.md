# `IsSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 解析器與腳本執行

## 描述

Whether the variable of the caller has a value. The tree carries it besides `IsSetRef`, and it asks for the variable and not for its value.

## 語法

```ahk
IsSet …
```

## 參數

the variable, not its value

## 傳回值

1 when the variable has a value, 0 when it has none

## 錯誤

an argument that is not a variable is refused with "IsSet requires a variable.".

## 範例

```ahk
x := 1
FileAppend IsSet(x), "*"
```
