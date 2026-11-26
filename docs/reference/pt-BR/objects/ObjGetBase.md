# `ObjGetBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · function · Os objetos da linguagem: Array, Map e Func

## Descrição

A base de um objeto: o protótipo da sua classe.

## Sintaxe

```ahk
ObjGetBase …
```

## Parâmetros

the object

## Valor de retorno

o objeto da base

## Erros

a value that is not an object is refused with "Expected an object but got ...".

## Exemplo

```ahk
FileAppend ObjGetBase({a: 1}).__Class, "*"
```
