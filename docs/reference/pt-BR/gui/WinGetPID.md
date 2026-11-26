# `WinGetPID`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Janelas da GUI e automação de janelas

## Descrição

Devolve o processo de uma janela, pelo barramento de acessibilidade da sessão: o registro do barramento nomeia os aplicativos e o barramento responde o PID.

## Sintaxe

```ahk
WinGetPID …
```

## Parâmetros

WinTitle

## Valor de retorno

o pid do processo, como inteiro

## Erros

"Target window not found." when no window has that title; an empty title is refused with "the window is found by its title, and an empty title is not carried yet."; the text of the window and the exclusions are refused with "the text of the window and the exclusions are not carried yet."

## Exemplo

```ahk
pid := WinGetPID("Calculator")
```
