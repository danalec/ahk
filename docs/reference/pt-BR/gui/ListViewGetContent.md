# `ListViewGetContent`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Janelas da GUI e automação de janelas

## Descrição

Reads the content of a ListView.

## Sintaxe

```ahk
S := ListViewGetContent(Options, Control, WinTitle)
```

## Parâmetros

0..6.

## Valor de retorno

The text.

## Exemplo

```ahk
ListViewGetContent("", "SysListView321")
```

## Relacionados

[`Gui`](Gui.md), [`ControlGetText`](ControlGetText.md)
