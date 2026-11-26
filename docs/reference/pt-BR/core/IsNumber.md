# `IsNumber`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Se um valor é um número: um inteiro, um float, ou um texto que contém um. Um texto hexadecimal também é número.

## Sintaxe

```ahk
B := IsNumber(Value)
```

## Parâmetros

- **Value** — Any value.

## Valor de retorno

1 quando é número, 0 quando não é

## Erros

a second parameter is refused with "the code page of the text is not carried yet.".

## Exemplo

```ahk
IsNumber("42")
```

## Relacionados

[`IsInteger`](IsInteger.md), [`IsFloat`](IsFloat.md)
