# `WinMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

Moves and resizes the window the script made; a Wayland compositor places the windows itself and need not honour the place a client asks for.

## Sözdizimi

```ahk
WinMove X, Y, W, H, WinTitle
```

## Parametreler

- **X/Y/W/H** — New geometry; omitted keeps.
- **WinTitle** — The criteria.

## Dönüş değeri

the empty text

## Hatalar

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## Örnek

```ahk
WinMove 100, 100, 800, 600, "MyWindow"
```

## İlgili

[`WinGetPos`](WinGetPos.md), [`WinMinimize`](WinMinimize.md)
