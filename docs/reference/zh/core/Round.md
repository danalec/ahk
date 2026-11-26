# `Round`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言核心：文本、数学与模式

## 描述

把数字舍入到调用方指定的小数位数。小数位数为正时返回的是文本，小数点后恰好是该位数的数字并保留零；不给、为 0 或为负时返回整数。

## 语法

```ahk
N := Round(Number, N)
```

## 参数

- **Number** — The value.
- **N** — Decimals; negative rounds tens/hundreds.

## 返回值

带有所需小数位的文本，或一个整数

## 错误

a text that does not hold a number is refused; more than 100000 decimals is an invalid parameter.

## 备注

Half-away-from-zero.

## 示例

```ahk
Round(2.567, 2)
```

## 相关

[`Floor`](Floor.md), [`Ceil`](Ceil.md), [`Mod`](Mod.md)
