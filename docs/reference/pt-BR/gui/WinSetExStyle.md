# `WinSetExStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Janelas da GUI e automação de janelas

## Descrição

Escreve o estilo estendido de uma janela. O estilo é absoluto, do mesmo jeito que o estilo de uma janela: o texto pode trazer um `+`, um `-` ou um `^` antes do número, e o runtime calcula o novo estilo estendido a partir do que já estava lá.

## Sintaxe

```ahk
WinSetExStyle …
```

## Parâmetros

Style, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Valor de retorno

texto vazio

## Erros

an empty text refuses with 'Parameter #1 of WinSetExStyle is invalid.', and 'empty string' is in the `Extra`; a window that is not there refuses with 'Target window not found.'.

## Exemplo

```ahk
WinSetExStyle("+0x8", "ZZZ_NO_WINDOW_ZZZ")
```
