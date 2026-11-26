# `WinExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · function · Okna GUI i automatyzacja okien

## Opis

Whether a window with this title exists. The windows are the ones of the compositor, and a number names one of them.

## Składnia

```ahk
HWND := WinExist(WinTitle, WinText, ExcludeTitle, ExcludeText)
```

## Parametry

- **WinTitle** — The criteria.

## Wartość zwracana

the identifier of the window that is there, zero when there is none

## Błędy

a window that is not there is an answer and not a failure; a question the compositor cannot answer fails with its reason.

## Uwagi

Sets the last-found window for the Win* family.

## Przykład

```ahk
if WinExist("Untitled")
```

## Powiązane

[`WinActive`](WinActive.md), [`WinActivate`](WinActivate.md)
