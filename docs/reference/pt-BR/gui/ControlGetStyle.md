# `ControlGetStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Janelas da GUI e automação de janelas

## Descrição

O estilo de um controle de uma janela: o número dos bits do Windows, do mesmo jeito que o estilo de uma janela. Um controle é uma janela por si só, e é achado pelo nome, pelo número ou pela classe.

## Sintaxe

```ahk
ControlGetStyle …
```

## Parâmetros

Control, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Valor de retorno

o estilo, como um número

## Erros

a window that is not there refuses with 'Target window not found.', and a control that is not there with 'Target control not found.'; the text of the window and the two exclusions are not carried yet.

## Exemplo

```ahk
FileAppend ControlGetStyle("Edit1", "ZZZ_NO_WINDOW_ZZZ"), "*"
```
