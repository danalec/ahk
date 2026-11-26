# `Throw`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*KeySharp* · function · 語言核心：文字、數學與模式

## 描述

A statement that raises a value. The value is usually an error object, and it may be any value.

## 語法

```ahk
Throw 
```

## 參數

Value

## 傳回值

nothing; the statement leaves the flow of the script

## 錯誤

none; the value is caught by the catch that names its class.

## 範例

```ahk
throw ValueError("boom")
```
