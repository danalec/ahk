# `Object`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Os objetos da linguagem: Array, Map e Func

## Descrição

A raiz da cadeia das classes: todo objeto do runtime é um. `Object()` cria um objeto vazio, e o literal `{a: 1}` cria um com uma propriedade.

## Sintaxe

```ahk
Obj := Object(Props...)
```

## Parâmetros

- **Props** — Alternating names and values.

## Valor de retorno

o objeto da classe `Object`

## Erros

nothing; a member that is not there is refused with the type of the value and the name of the member.

## Observações

Base is Object; OwnProps/HasProp/DefineProp/DeleteProp; __Get/__Set/__Call meta-functions.

## Exemplo

```ahk
o := Object()
FileAppend o.__Class, "*"
```

## Relacionados

[`Map`](Map.md), [`Array`](Array.md), [`Gui`](Gui.md)
