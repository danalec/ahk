# `RTrim`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

O texto sem os caracteres da ponta direita.

## Sintaxe

```ahk
S := RTrim(String, OmitChars)
```

## Parâmetros

- **String** — The text.
- **OmitChars** — Default space+tab.

## Valor de retorno

o texto sem os caracteres finais do conjunto; o conjunto é o espaço e o tab quando quem chama não nomeia um

## Erros

none.

## Exemplo

```ahk
RTrim("  hello  ")
```

## Relacionados

[`Trim`](Trim.md), [`LTrim`](LTrim.md)
