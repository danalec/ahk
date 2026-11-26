# `NumGet`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · function · Native function calls (DllCall)

## Description

Reads a number out of the bytes of a buffer; the source of a number is a buffer, and the memory of an address is not read here.

## Syntax

```ahk
N := NumGet(Source, Offset, Type)
```

## Parameters

- **Source** — A Buffer/pointer.
- **Offset** — Bytes.
- **Type** — UInt/Int/Float/Ptr/etc.

## Return value

the number the bytes hold

## Errors

"this runtime does not read the memory of an address: the source of a number is a buffer." when the source is a number; a type that is not one of the eleven is refused with "the type of a number is one of UInt, Int, Int64, Short, UShort, Char, UChar, Double, Float, Ptr or UPtr."; bytes that do not fit are "the bytes do not fit in the buffer."

## Example

```ahk
value := NumGet(buf, 0, "Int")
```

## Related

[`NumPut`](NumPut.md), [`Buffer`](Buffer.md), [`StrGet`](StrGet.md)
