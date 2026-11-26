# `Click`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Skróty, ciągi wyzwalające i wprowadzanie

## Opis

Clicks with the pointer of the backend, which is an absolute uinput device: it moves to the place, settles, presses and releases.

## Składnia

```ahk
Click Options
```

## Parametry

- **Options** — X Y, button, count, Down/Up, Rel.

## Wartość zwracana

nothing

## Błędy

the wheel, the fourth and the fifth buttons, the pressing of a button and the place relative to the pointer are refused as not carried yet; on a host without `grim` the click fails with the reason of the screen, because the pointer needs the size of the screen.

## Uwagi

On Linux the absolute uinput pointer / XTEST.

## Przykład

```ahk
Click 100, 200
```

## Powiązane

[`MouseMove`](MouseMove.md), [`MouseClick`](MouseClick.md), [`Send`](Send.md)
