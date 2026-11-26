# `WinShow`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Janelas da GUI e automação de janelas

## Descrição

Põe a janela do script de volta na tela, depois de um `WinHide`.

## Sintaxe

```ahk
WinShow …
```

## Parâmetros

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Valor de retorno

nada

## Erros

a window that is not there answers 'Target window not found.'; a window of another program cannot be shown this way.

## Exemplo

```ahk
WinShow "Untitled"
```
