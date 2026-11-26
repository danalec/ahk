# `WinExist`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · Ventanas de la GUI y automatización de ventanas

## Descripción

Whether a window with this title exists. The windows are the ones of the compositor, and a number names one of them.

## Sintaxis

```ahk
HWND := WinExist(WinTitle, WinText, ExcludeTitle, ExcludeText)
```

## Parámetros

- **WinTitle** — The criteria.

## Valor de retorno

the identifier of the window that is there, zero when there is none

## Errores

a window that is not there is an answer and not a failure; a question the compositor cannot answer fails with its reason.

## Observaciones

Sets the last-found window for the Win* family.

## Ejemplo

```ahk
if WinExist("Untitled")
```

## Relacionados

[`WinActive`](WinActive.md), [`WinActivate`](WinActivate.md)
