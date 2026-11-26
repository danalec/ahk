# `Map`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*KeySharp* · method · `Array` · Os objetos da linguagem: Array, Map e Func

## Descrição

A classe de uma tabela de chaves e valores. `Map(key, value, ...)` cria uma, e a ordem das entradas é a ordem em que foram colocadas.

## Sintaxe

```ahk
M := Map(Key1, Value1, ...)
```

## Parâmetros

- **Pairs** — Alternating keys and values.

## Valor de retorno

o objeto da classe `Map`

## Erros

an odd number of arguments is refused with "Invalid number of parameters."; a key that is not there is refused with "Item has no value.".

## Observações

Backed by a BTreeMap on this tree (ordered, comparison-based). CaseSense/Default properties; Get/Set/Has/Delete/Clear/Count/__Enum.

## Exemplo

```ahk
m := Map("x", 1)
FileAppend m.Count, "*"
```

## Relacionados

[`Array`](Array.md), [`Object`](Object.md)
