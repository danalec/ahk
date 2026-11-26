# `NumPut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Chamadas a funções nativas (DllCall)

## Descrição

Escreve números nos bytes de um buffer; os tipos e os números vêm em pares e o alvo vem depois deles, como o v2 escreve.

## Sintaxe

```ahk
NumPut Type, Number, Target, Offset
```

## Parâmetros

- **Type** — The numeric type.
- **Number** — The value.
- **Target** — Buffer/pointer.
- **Offset** — Bytes.

## Valor de retorno

o endereço à direita do último número escrito

## Erros

"this runtime does not write to the memory of an address: the target of a number is a buffer." when the target is a number; a number of types that is not even is "the types and the numbers of a write come in pairs."; anything after the target and the offset is "a write takes the target and the offset after the pairs."

## Exemplo

```ahk
NumPut("Int", 42, buf, 0)
```

## Relacionados

[`NumGet`](NumGet.md), [`Buffer`](Buffer.md)
