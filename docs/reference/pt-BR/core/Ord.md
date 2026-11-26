# `Ord`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

O ponto de código do primeiro caractere de um texto.

## Sintaxe

```ahk
N := Ord(String)
```

## Parâmetros

- **String** — The text.

## Valor de retorno

o ponto de código, ou zero quando o texto não tem caractere

## Erros

none; a lone surrogate answers with the code unit of it.

## Exemplo

```ahk
Ord("A")
```

## Relacionados

[`Chr`](Chr.md), [`Asc`](Asc.md)
