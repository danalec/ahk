# `NumPut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Вызовы нативных функций (DllCall)

## Описание

Writes numbers into the bytes of a buffer; the types and the numbers come in pairs and the target comes after them, as v2 writes it.

## Синтаксис

```ahk
NumPut Type, Number, Target, Offset
```

## Параметры

- **Type** — The numeric type.
- **Number** — The value.
- **Target** — Buffer/pointer.
- **Offset** — Bytes.

## Возвращаемое значение

the address to the right of the last number that was written

## Ошибки

"this runtime does not write to the memory of an address: the target of a number is a buffer." when the target is a number; a number of types that is not even is "the types and the numbers of a write come in pairs."; anything after the target and the offset is "a write takes the target and the offset after the pairs."

## Пример

```ahk
NumPut("Int", 42, buf, 0)
```

## См. также

[`NumGet`](NumGet.md), [`Buffer`](Buffer.md)
