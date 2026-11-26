# `RegExReplace`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

O texto com as ocorrências de um padrão substituídas. A substituição pode nomear os grupos do padrão com $1 ou \1, e $$ é um cifrão de verdade.

## Sintaxe

```ahk
RegExReplace …
```

## Parâmetros

Text, Pattern, Replacement, [&Count], [Limit], [Start]

## Valor de retorno

o texto com as substituições

## Erros

the count must be a variable reference; a pattern that does not compile fails with the reason of the compiler.

## Exemplo

```ahk
RegExReplace("a1b2", "\d", "#")
```
