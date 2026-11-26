# `WinGetTransColor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Janelas da GUI e automação de janelas

## Descrição

A cor que a janela deixa passar: a chave de cor de uma janela em camadas, a que não é desenhada. A cor é escrita como "0xRRGGBB", na ordem dos bits do vermelho, do verde e do azul. Uma janela que não é em camadas responde o texto vazio.

## Sintaxe

```ahk
WinGetTransColor …
```

## Parâmetros

WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Valor de retorno

a cor como "0xRRGGBB", ou o texto vazio quando a janela não é em camadas

## Erros

a window that is not there refuses with 'Target window not found.', and the title that was asked about is in the `Extra`; the text of the window and the two exclusions are not carried yet.

## Exemplo

```ahk
FileAppend WinGetTransColor("ZZZ_NO_WINDOW_ZZZ"), "*"
```
