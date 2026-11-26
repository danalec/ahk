# `NumPut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 原生函数调用 (DllCall)

## 描述

把若干数写进缓冲区的字节；类型和数成对出现，目标在它们之后，和 v2 的写法一样。

## 语法

```ahk
NumPut Type, Number, Target, Offset
```

## 参数

- **Type** — The numeric type.
- **Number** — The value.
- **Target** — Buffer/pointer.
- **Offset** — Bytes.

## 返回值

写下去的最后一个数右边的地址

## 错误

"this runtime does not write to the memory of an address: the target of a number is a buffer." when the target is a number; a number of types that is not even is "the types and the numbers of a write come in pairs."; anything after the target and the offset is "a write takes the target and the offset after the pairs."

## 示例

```ahk
NumPut("Int", 42, buf, 0)
```

## 相关

[`NumGet`](NumGet.md), [`Buffer`](Buffer.md)
