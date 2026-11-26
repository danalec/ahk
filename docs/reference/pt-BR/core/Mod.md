# `Mod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

O resto de uma divisão. O sinal do resto é o do dividendo, e a resposta é um número inteiro quando os dois argumentos são.

## Sintaxe

```ahk
R := Mod(Dividend, Divisor)
```

## Parâmetros

- **Dividend** — The number.
- **Divisor** — Non-zero.

## Valor de retorno

o resto

## Erros

a divisor of zero fails with "Divide by zero."; a value that is not a number is refused with "Invalid parameter(s).".

## Observações

A zero divisor throws ZeroDivisionError.

## Exemplo

```ahk
Mod(7, 3)
```

## Relacionados

[`Abs`](Abs.md), [`Round`](Round.md), [`//`](__.md)
