# `Random`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 語言核心：文字、數學與模式

## 描述

A number between the two the caller names. Without arguments it is a number between zero and one; with one it is a whole number below it; with two it is a whole number between them, both ends counted.

## 語法

```ahk
N := Random(Min, Max)
```

## 參數

- **Min/Max** — The range; omitted = float 0..1.
- **Seed** — With a single arg, reseeds.

## 傳回值

the number

## 錯誤

none.

## 範例

```ahk
Random(1, 6)
```

## 相關

[`Sort`](Sort.md)
