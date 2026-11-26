# `Array`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · class · Os objetos da linguagem: Array, Map e Func

## Descrição

A classe de uma fileira de valores. `Array(...)` cria uma com os valores dados, e o literal `[1, 2, 3]` cria uma também.

## Sintaxe

```ahk
Arr := Array(Values...)
```

## Parâmetros

- **Values** — The initial elements.

## Valor de retorno

o objeto da classe `Array`

## Erros

nothing to make the array; the places count from one, and an index that is not there is refused with "Invalid index.".

## Observações

Methods: Push/Pop/InsertAt/RemoveAt/Get/Set/Length/Capacity/Clone/__Enum.

## Exemplo

```ahk
a := Array(1, 2, 3)
FileAppend a.Length, "*"
```

## Relacionados

[`Map`](Map.md), [`Object`](Object.md), [`StrSplit`](StrSplit.md)
