# `SendMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

Sends a message to a window and waits.

## Sözdizimi

```ahk
R := SendMessage(Msg, wParam, lParam, Control, WinTitle, Timeout)
```

## Parametreler

1..9.

## Dönüş değeri

The result.

## Örnek

```ahk
SendMessage(0x100, 0, 0)
```

## İlgili

[`Menu`](Menu.md), [`Gui`](Gui.md), [`OnMessage`](OnMessage.md)
