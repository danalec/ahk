# `WinClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Janelas da GUI e automação de janelas

## Descrição

Fecha uma janela. Fecha a janela do próprio script: a listagem do compositor não publica pedido de fechamento para a janela de outro programa.

## Sintaxe

```ahk
WinClose WinTitle, WinText, SecondsToWait
```

## Parâmetros

- **WinTitle** — The criteria.
- **SecondsToWait** — Then force.

## Valor de retorno

nada

## Erros

a window that is not there answers 'Target window not found.'; a window of another program cannot be closed this way.

## Exemplo

```ahk
WinClose "Untitled"
```

## Relacionados

[`WinKill`](WinKill.md), [`WinActivate`](WinActivate.md)
