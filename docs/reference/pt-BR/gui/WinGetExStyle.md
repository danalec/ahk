# `WinGetExStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Janelas da GUI e automação de janelas

## Descrição

O estilo estendido de uma janela: os bits `WS_EX_*` do Windows, os que dizem como a janela lida com as outras janelas e com a tela. O estilo estendido da janela do próprio script é 256.

## Sintaxe

```ahk
WinGetExStyle …
```

## Parâmetros

WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Valor de retorno

o estilo estendido, como um número

## Erros

a window that is not there refuses with 'Target window not found.', and the title that was asked about is in the `Extra`; the text of the window and the two exclusions are not carried yet.

## Exemplo

```ahk
FileAppend WinGetExStyle("ZZZ_NO_WINDOW_ZZZ"), "*"
```
