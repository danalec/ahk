# `StrSplit`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

As partes de um texto, cortadas nos delimitadores. Um delimitador é literal: um texto é um delimitador, um array de textos é um conjunto deles, e o primeiro que estiver presente vence.

## Sintaxe

```ahk
Array := StrSplit(String, Delimiters, OmitChars, MaxParts)
```

## Parâmetros

- **String** — The text.
- **Delimiters** — A string of delimiter chars or an array.
- **OmitChars** — Chars to trim from each part.
- **MaxParts** — Limit the number of parts.

## Valor de retorno

um array das partes; os caracteres a omitir são cortados do começo e do fim de cada parte

## Erros

the delimiters must be a text or an array of texts; an empty array is refused with "Parameter #2 of StrSplit is invalid.".

## Observações

Empty delimiters split into characters.

## Exemplo

```ahk
StrSplit("a,b,c", ",")
```

## Relacionados

[`StrReplace`](StrReplace.md), [`Array`](Array.md), [`LoopParse`](LoopParse.md)
