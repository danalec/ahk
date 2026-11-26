# `Chr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

O caractere com um ponto de código.

## Sintaxe

```ahk
S := Chr(Number)
```

## Parâmetros

- **Number** — The code point.

## Valor de retorno

o texto desse caractere

## Erros

a code outside 0 to 0x10FFFF is an invalid parameter; a surrogate code is a text of its own.

## Exemplo

```ahk
Chr(65)
```

## Relacionados

[`Ord`](Ord.md), [`Asc`](Asc.md)
