# `FormatTime`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Datas e horas

## Descrição

Um momento escrito do jeito que o formato diz. A data vem primeiro e o formato por último; um texto sem data nenhuma é a data da máquina.

## Sintaxe

```ahk
S := FormatTime(YYYYMMDDHH24MISS, Format)
```

## Parâmetros

- **Time** — A timestamp; omitted = now.
- **Format** — The picture (yyyy-MM-dd etc.).

## Valor de retorno

a data escrita do jeito que o formato diz

## Erros

a date the dialect cannot read is an invalid parameter.

## Exemplo

```ahk
FormatTime(A_Now, "yyyy-MM-dd")
```

## Relacionados

[`DateAdd`](DateAdd.md), [`DateDiff`](DateDiff.md), [`A_Now`](A_Now.md)
