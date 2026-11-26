# `Enumerator`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · class · Os objetos da linguagem: Array, Map e Func

## Descrição

A classe do andarilho de um laço: um enumerador entrega as partes de uma sequência, uma volta por vez. A classe não é criada; o `__Enum` de um valor cria uma, e a chamada dela é a volta da enumeração.

## Sintaxe

```ahk
Enumerator 
```

## Parâmetros

nothing that makes it

## Valor de retorno

a classe de um enumerador, como um texto

## Erros

the call of the class is refused with "Invalid base.".

## Exemplo

```ahk
for item in [1, 2]
    FileAppend Type(item), "*"
```
