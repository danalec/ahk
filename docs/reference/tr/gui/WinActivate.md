# `WinActivate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

Brings a window to the front. It acts on the window of the script itself: the listing of the compositor publishes no way to raise a window of another program.

## Sözdizimi

```ahk
WinActivate WinTitle, WinText, ExcludeTitle, ExcludeText
```

## Parametreler

- **WinTitle** — Title/ahk_class/ahk_exe/ahk_id/ahk_pid.
- **WinText** — A text the window must contain.
- **Exclude*** — Criteria to skip.

## Dönüş değeri

nothing

## Hatalar

a window that is not there answers 'Target window not found.'; a window of another program cannot be activated this way.

## Notlar

On Wayland the compositor may refuse focus; the tree reports the reason.

## Örnek

```ahk
WinActivate "Untitled"
```

## İlgili

[`WinWait`](WinWait.md), [`WinActive`](WinActive.md), [`WinClose`](WinClose.md)
