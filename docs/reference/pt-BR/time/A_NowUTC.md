# `A_NowUTC`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · Datas e horas

## Descrição

A hora da máquina em UTC, escrita como YYYYMMDDHH24MISS.

## Sintaxe

```ahk
A_NowUTC 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

a hora, como um texto de quatorze dígitos

## Erros

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Exemplo

```ahk
FileAppend A_NowUTC, "*"
```
