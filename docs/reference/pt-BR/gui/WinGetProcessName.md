# `WinGetProcessName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Janelas da GUI e automação de janelas

## Descrição

Devolve o nome do programa de uma janela; ela segue a cadeia de WinGetPID: a janela nomeia um aplicativo no barramento, o barramento responde o processo e a tabela de processos responde o nome.

## Sintaxe

```ahk
WinGetProcessName …
```

## Parâmetros

WinTitle

## Valor de retorno

o nome do arquivo do executável, como texto

## Erros

"Target window not found." when no window has that title, "Target process not found." when the process is gone by then; an empty title is refused with "the window is found by its title, and an empty title is not carried yet."

## Exemplo

```ahk
name := WinGetProcessName("Calculator")
```
