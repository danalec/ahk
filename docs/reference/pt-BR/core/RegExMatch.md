# `RegExMatch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Onde um padrão está num texto. A posição conta a partir de um e é zero quando o padrão não está lá.

## Sintaxe

```ahk
RegExMatch …
```

## Parâmetros

Text, Pattern, [&Output], [Start]

## Valor de retorno

a posição da ocorrência, ou 0 quando não há nenhuma

## Erros

the output must be a variable reference; the output holds an object of the class RegExMatchInfo, and a text that does not match leaves it holding the empty text.

## Exemplo

```ahk
RegExMatch("abc123", "\d+")
```
