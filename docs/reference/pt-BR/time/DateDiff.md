# `DateDiff`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Datas e horas

## Descrição

Devolve a diferença entre dois momentos; a resposta é a primeira data menos a segunda, truncada para zero.

## Sintaxe

```ahk
DateDiff …
```

## Parâmetros

DateTime1, DateTime2, Unit

## Valor de retorno

a diferença, como inteiro, na unidade pedida

## Erros

"Parameter #3 of DateDiff is invalid." when the unit is not one of d, h, m and s; a date that v2 cannot read is an invalid parameter.

## Exemplo

```ahk
days := DateDiff(20240110, 20240101, "d")
```
