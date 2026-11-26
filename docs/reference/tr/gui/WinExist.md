# `WinExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · GUI pencereleri ve pencere otomasyonu

## Açıklama

Whether a window with this title exists. The windows are the ones of the compositor, and a number names one of them.

## Sözdizimi

```ahk
HWND := WinExist(WinTitle, WinText, ExcludeTitle, ExcludeText)
```

## Parametreler

- **WinTitle** — The criteria.

## Dönüş değeri

the identifier of the window that is there, zero when there is none

## Hatalar

a window that is not there is an answer and not a failure; a question the compositor cannot answer fails with its reason.

## Notlar

Sets the last-found window for the Win* family.

## Örnek

```ahk
if WinExist("Untitled")
```

## İlgili

[`WinActive`](WinActive.md), [`WinActivate`](WinActivate.md)
