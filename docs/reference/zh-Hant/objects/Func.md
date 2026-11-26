# `Func`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 語言的物件：Array、Map 與 Func

## 描述

The class of a function: a function of the script, a method, a lambda or a built-in. The class is not made: `Func(...)` answers "Invalid base."; the class of a function value is `Func`.

## 語法

```ahk
Func …
```

## 參數

nothing that makes it

## 傳回值

the class of a function value, as a text

## 錯誤

the call of the class is refused with "Invalid base.".

## 範例

```ahk
FileAppend Abs.__Class, "*"
```
