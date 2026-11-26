# `TrayTip`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Janelas da GUI e automação de janelas

## Descrição

Manda uma notificação à área de trabalho com um texto e um título. O nome é do Windows -- um balão na área de notificação -- mas a árvore manda uma notificação de mesa, que chega ao mesmo usuário com as mesmas palavras.

## Sintaxe

```ahk
TrayTip Text, Title, Options
```

## Parâmetros

- **Text** — The message.
- **Title** — The title.
- **Options** — Icon/mute.

## Valor de retorno

texto vazio

## Erros

the desktop reports the failure of the notification.

## Exemplo

```ahk
TrayTip "done", "the script"
```

## Relacionados

[`ToolTip`](ToolTip.md), [`MsgBox`](MsgBox.md)
