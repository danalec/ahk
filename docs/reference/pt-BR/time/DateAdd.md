# `DateAdd`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Datas e horas

## Descrição

Devolve um momento mais um tempo; a data vem primeiro e a unidade por último, e a unidade é lida pela primeira letra.

## Sintaxe

```ahk
DateAdd …
```

## Parâmetros

DateTime, Number, Unit

## Valor de retorno

a nova data, como o texto YYYYMMDDHH24MISS

## Erros

"Parameter #3 of DateAdd is invalid." when the unit is not one of d, h, m and s; "Parameter #1 of DateAdd is invalid." when the date is not one that v2 can read; "Parameter #2 of DateAdd requires a Number, but received a String."

## Exemplo

```ahk
next := DateAdd(20240101, 1, "d")
```
