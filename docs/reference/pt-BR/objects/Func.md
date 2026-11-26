# `Func`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Os objetos da linguagem: Array, Map e Func

## Descrição

A classe de uma função: uma função do script, um método, um lambda ou um embutido. A classe não é criada: `Func(...)` responde "Invalid base."; a classe de um valor de função é `Func`.

## Sintaxe

```ahk
Func …
```

## Parâmetros

nothing that makes it

## Valor de retorno

a classe de um valor de função, como um texto

## Erros

the call of the class is refused with "Invalid base.".

## Exemplo

```ahk
FileAppend Abs.__Class, "*"
```
