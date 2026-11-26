# `SubStr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

A parte de um texto entre duas posições. As posições contam a partir de um, e uma posição negativa conta a partir do fim: -1 é o último caractere.

## Sintaxe

```ahk
Part := SubStr(String, Start, Length)
```

## Parâmetros

- **String** — The text.
- **Start** — 1-based; negative counts from the end.
- **Length** — Omitted = to the end.

## Valor de retorno

o texto dessa parte; um comprimento negativo deixa tantos caracteres de fora no fim

## Erros

none; the text is cut in the code units of UTF-16, so the second unit of a surrogate pair is a text of its own.

## Observações

Counts characters, not bytes.

## Exemplo

```ahk
SubStr("hello", 2, 3)
```

## Relacionados

[`StrLen`](StrLen.md), [`InStr`](InStr.md), [`StrSplit`](StrSplit.md)
