# `A_Mon`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · Datas e horas

## Descrição

O mês da hora local da máquina. Esta árvore responde os mesmos dois dígitos de `A_MM`, e não o nome abreviado do mês.

## Sintaxe

```ahk
A_Mon 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o mês, como um texto de dois dígitos

## Erros

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Exemplo

```ahk
FileAppend A_Mon, "*"
```
