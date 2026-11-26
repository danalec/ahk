# `ControlFindItem`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

Finds an item by text.

## Sözdizimi

```ahk
N := ControlFindItem(Text, Control, WinTitle)
```

## Parametreler

2..6.

## Dönüş değeri

The index or 0.

## Örnek

```ahk
ControlFindItem("x", "ListBox1")
```

## İlgili

[`ControlSend`](ControlSend.md), [`Gui`](Gui.md)
