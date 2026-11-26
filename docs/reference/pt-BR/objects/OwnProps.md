# `OwnProps`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · method · `Object` · Os objetos da linguagem: Array, Map e Func

## Descrição

Iterates the own properties of an object.

## Sintaxe

```ahk
for name, value in Obj.OwnProps()
```

## Parâmetros

?.

## Valor de retorno

The enumerator.

## Exemplo

```ahk
for n, v in o.OwnProps()
```

## Relacionados

[`HasOwnProp`](HasOwnProp.md), [`DefineProp`](DefineProp.md)
