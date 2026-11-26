# `WinSetStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Janelas da GUI e automação de janelas

## Descrição

Escreve o estilo de uma janela. O estilo é absoluto: o texto pode trazer um `+`, um `-` ou um `^` antes do número, e o runtime calcula o novo estilo a partir do que já estava lá -- `+0x10` soma os bits, `-0x10` tira eles, `^0x10` os inverte, e um texto sem prefixo substitui o estilo. O número é lido do jeito que o `ATOU` do v2 lê: hexadecimal quando o texto diz "0x", decimal nos outros casos, e zero quando o texto não tem número.

## Sintaxe

```ahk
WinSetStyle …
```

## Parâmetros

Style, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Valor de retorno

texto vazio

## Erros

an empty text refuses with 'Parameter #1 of WinSetStyle is invalid.', and 'empty string' is in the `Extra`; a text that holds no number is not a refusal -- it is read as zero, and the failure that comes is the one of the window.

## Exemplo

```ahk
WinSetStyle("+0x10", "ZZZ_NO_WINDOW_ZZZ")
```
