# `NumGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 原生函数调用 (DllCall)

## 描述

从缓冲区的字节里读出一个数；数的来源是缓冲区，这里不读取某个地址的内存。

## 语法

```ahk
N := NumGet(Source, Offset, Type)
```

## 参数

- **Source** — A Buffer/pointer.
- **Offset** — Bytes.
- **Type** — UInt/Int/Float/Ptr/etc.

## 返回值

这些字节所持有的数

## 错误

"this runtime does not read the memory of an address: the source of a number is a buffer." when the source is a number; a type that is not one of the eleven is refused with "the type of a number is one of UInt, Int, Int64, Short, UShort, Char, UChar, Double, Float, Ptr or UPtr."; bytes that do not fit are "the bytes do not fit in the buffer."

## 示例

```ahk
value := NumGet(buf, 0, "Int")
```

## 相关

[`NumPut`](NumPut.md), [`Buffer`](Buffer.md), [`StrGet`](StrGet.md)
