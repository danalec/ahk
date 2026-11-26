# `GetMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · method · `Object` · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Um método de um objeto, como um valor que pode ser chamado.

## Sintaxe

```ahk
GetMethod 
```

## Parâmetros

the object, the name of the method, and the number of parameters it would take

## Valor de retorno

o método, como um valor da classe `Func`

## Erros

a method that is not there is refused with the type of the value and the name of the member.

## Exemplo

```ahk
o := {a: 1}
FileAppend GetMethod(o, "HasOwnProp").__Class, "*"
```
