# `Sleep`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · O analisador e a execução do script

## Descrição

Pausa o script por um número de milissegundos.

## Sintaxe

```ahk
Sleep Milliseconds
```

## Parâmetros

- **Milliseconds** — 0..60000; a larger value is clamped to 60000, a negative one sleeps 0.

## Valor de retorno

o número de milissegundos esperado

## Erros

none; a wait longer than 60000 milliseconds is cut at that number.

## Observações

The pause is clamped, not refused. During the sleep the timers of the script may still run (the runtime checks them between statements).

## Exemplo

```ahk
Sleep 500
```

## Relacionados

[`SetTimer`](SetTimer.md), [`Critical`](Critical.md)
