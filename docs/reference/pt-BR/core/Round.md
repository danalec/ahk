# `Round`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

O número arredondado ao número de decimais pedido. Com um número positivo de decimais a resposta é um texto com exatamente esse tanto de dígitos depois do ponto, zeros incluidos; sem o número, com zero ou negativo, é um inteiro.

## Sintaxe

```ahk
N := Round(Number, N)
```

## Parâmetros

- **Number** — The value.
- **N** — Decimals; negative rounds tens/hundreds.

## Valor de retorno

um texto com os decimais pedidos, ou um inteiro

## Erros

a text that does not hold a number is refused; more than 100000 decimals is an invalid parameter.

## Observações

Half-away-from-zero.

## Exemplo

```ahk
Round(2.567, 2)
```

## Relacionados

[`Floor`](Floor.md), [`Ceil`](Ceil.md), [`Mod`](Mod.md)
