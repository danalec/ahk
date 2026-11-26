# `Format`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Um padrão com os valores dentro. O padrão é o do dialeto de referência: {1} é o primeiro argumento, um placeholder sem número pega o próximo, e {{ e }} são uma chave cada.

## Sintaxe

```ahk
Text := Format(FormatStr, Values...)
```

## Parâmetros

- **FormatStr** — With {1}, {2:0.00}, {:#x} placeholders.
- **Values** — The values to insert.

## Valor de retorno

o texto do padrão com os valores dentro

## Erros

a placeholder that names an argument that is not there, or whose spec is not one the dialect reads, is left exactly as it was written; a value of a kind the spec does not want is refused.

## Observações

Supports width, precision, hex, and alignment.

## Exemplo

```ahk
Format("{1} + {2} = {3}", 1, 2, 3)
```

## Relacionados

[`StrReplace`](StrReplace.md), [`String`](String.md)
