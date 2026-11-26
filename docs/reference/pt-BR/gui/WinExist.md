# `WinExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Janelas da GUI e automação de janelas

## Descrição

Se existe uma janela com este título. As janelas são as do compositor, e um número nomeia uma delas.

## Sintaxe

```ahk
HWND := WinExist(WinTitle, WinText, ExcludeTitle, ExcludeText)
```

## Parâmetros

- **WinTitle** — The criteria.

## Valor de retorno

o identificador da janela que existe, zero quando não existe

## Erros

a window that is not there is an answer and not a failure; a question the compositor cannot answer fails with its reason.

## Observações

Sets the last-found window for the Win* family.

## Exemplo

```ahk
if WinExist("Untitled")
```

## Relacionados

[`WinActive`](WinActive.md), [`WinActivate`](WinActivate.md)
