# `Random`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 语言核心：文本、数学与模式

## 描述

调用方指定两个数之间的数字。不带参数时是 0 与 1 之间的数；带一个参数是小于它的整数；带两个参数是两者之间的整数，两端都算在内。

## 语法

```ahk
N := Random(Min, Max)
```

## 参数

- **Min/Max** — The range; omitted = float 0..1.
- **Seed** — With a single arg, reseeds.

## 返回值

该数字

## 错误

none.

## 示例

```ahk
Random(1, 6)
```

## 相关

[`Sort`](Sort.md)
