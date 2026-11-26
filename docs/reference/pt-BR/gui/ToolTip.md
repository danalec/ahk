# `ToolTip`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Janelas da GUI e automação de janelas

## Descrição

Mostra uma pequena janela de texto num ponto da tela. Chamada sem texto, ela remove a janela.

## Sintaxe

```ahk
ToolTip Text, X, Y, WhichToolTip
```

## Parâmetros

- **Text** — The text; omitted hides.
- **X/Y** — Position.
- **WhichToolTip** — 1..20.

## Valor de retorno

o número que identifica a janela, ou texto vazio quando a janela é removida

## Erros

the desktop reports the failure of the call.

## Exemplo

```ahk
ToolTip "hello", 10, 10
```

## Relacionados

[`TrayTip`](TrayTip.md), [`MsgBox`](MsgBox.md)
