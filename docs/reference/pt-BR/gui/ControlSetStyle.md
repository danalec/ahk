# `ControlSetStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Janelas da GUI e automação de janelas

## Descrição

Escreve o estilo de um controle de uma janela. O estilo é absoluto, do mesmo jeito que o estilo de uma janela: o texto pode trazer um `+`, um `-` ou um `^` antes do número, e o runtime calcula o novo estilo a partir do que já estava lá.

## Sintaxe

```ahk
ControlSetStyle …
```

## Parâmetros

Style, Control, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Valor de retorno

texto vazio

## Erros

an empty text refuses with 'Parameter #1 of ControlSetStyle is invalid.'; a window that is not there refuses with 'Target window not found.', and a control that is not there with 'Target control not found.'.

## Exemplo

```ahk
ControlSetStyle("+0x10", "Edit1", "ZZZ_NO_WINDOW_ZZZ")
```
