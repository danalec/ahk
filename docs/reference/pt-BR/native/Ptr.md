# `Ptr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · method · `Buffer` · Chamadas a funções nativas (DllCall)

## Descrição

The pointer type marker for DllCall/NumGet (the machine's pointer width).

## Sintaxe

```ahk
Ptr
```

## Parâmetros

?.

## Valor de retorno

Used as a type name; 8 bytes on 64-bit, 4 on 32-bit.

## Exemplo

```ahk
NumGet(buf, 0, "Ptr")
```

## Relacionados

[`PtrSize`](PtrSize.md), [`Buffer`](Buffer.md), [`DllCall`](DllCall.md)
