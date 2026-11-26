# `WinSetTransColor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Janelas da GUI e automação de janelas

## Descrição

Escreve a cor que a janela deixa passar. A cor é o nome de uma das dezessete cores do HTML, um número hexadecimal, ou a palavra "Off", que tira a chave de cor.

## Sintaxe

```ahk
WinSetTransColor …
```

## Parâmetros

Color, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Valor de retorno

texto vazio

## Erros

a text that is not a colour refuses with 'Parameter #1 of WinSetTransColor is invalid.', and the text is in the `Extra`; a window that is not there refuses with 'Target window not found.'.

## Exemplo

```ahk
WinSetTransColor("Red", "ZZZ_NO_WINDOW_ZZZ")
```
