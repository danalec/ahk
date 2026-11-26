# `WinGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Janelas da GUI e automação de janelas

## Descrição

Onde a janela está e quão grande ela é, como o compositor relata.

## Sintaxe

```ahk
WinGetPos X, Y, W, H, WinTitle
```

## Parâmetros

- **Output*** — Variables.
- **WinTitle** — The criteria.

## Valor de retorno

nada; o lugar e o tamanho são escritos nas variáveis de quem chama

## Erros

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## Exemplo

```ahk
WinGetPos "Untitled", , &x, &y, &w, &h
```

## Relacionados

[`WinMove`](WinMove.md), [`WinGetTitle`](WinGetTitle.md)
