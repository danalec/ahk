# `Bind`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · method · `Func` · Os objetos da linguagem: Array, Map e Func

## Descrição

Binds arguments to a function, answering a BoundFunc.

## Sintaxe

```ahk
F := Func.Bind(Args...)
```

## Parâmetros

?.

## Valor de retorno

A BoundFunc.

## Exemplo

```ahk
f := Func.Bind(1)
```

## Relacionados

[`BoundFunc`](BoundFunc.md), [`ObjBindMethod`](ObjBindMethod.md)
