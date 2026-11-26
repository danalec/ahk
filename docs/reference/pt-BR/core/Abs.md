# `Abs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

A distância até zero. Um inteiro continua inteiro.

## Sintaxe

```ahk
N := Abs(Number)
```

## Parâmetros

- **Number** — Integer or float.

## Valor de retorno

o número sem o seu sinal

## Erros

a value that is not a number is refused.

## Observações

Scalar; no SIMD gain (docs/simd.md).

## Exemplo

```ahk
Abs(-3)
```

## Relacionados

[`Mod`](Mod.md), [`Round`](Round.md), [`Min`](Min.md), [`Max`](Max.md)
