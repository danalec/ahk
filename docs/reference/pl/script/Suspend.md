# `Suspend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Parser i wykonanie skryptu

## Opis

Turns the hotkeys of the script off or on. With no parameter it turns them the other way. The state is visible in A_IsSuspended and in the enabled flag of the hotkey table.

## Składnia

```ahk
Suspend …
```

## Parametry

[OnOff]

## Wartość zwracana

the empty text

## Błędy

a parameter that is not 0, 1 or -1 is an invalid parameter.

## Przykład

```ahk
Suspend true
```
