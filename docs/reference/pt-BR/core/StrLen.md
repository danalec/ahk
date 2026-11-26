# `StrLen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Conta os caracteres de um texto em unidades de código UTF-16, então um caractere fora do plano básico conta como dois.

## Sintaxe

```ahk
Count := StrLen(String)
```

## Parâmetros

- **String** — The text.

## Valor de retorno

o número de unidades de código

## Erros

none; a value that is not text is read as its text.

## Observações

Astral characters count as one each on this tree.

## Exemplo

```ahk
StrLen("hello")
```

## Relacionados

[`SubStr`](SubStr.md), [`InStr`](InStr.md)
