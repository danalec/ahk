# `SoundGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Dźwięk

## Opis

Reads a sound component/control (v1 shape).

## Składnia

```ahk
SoundGet OutputVar, Component, Control, Device
```

## Parametry

- **OutputVar** — Receives the value.
- **Component/Control/Device** — The mixer selectors.

## Wartość zwracana

Nothing; the var is filled.

## Uwagi

On Linux pactl; the Windows mixer concepts map to volume/mute.

## Przykład

```ahk
SoundGet &v, , "Volume"
```

## Powiązane

[`SoundSet`](SoundSet.md), [`SoundGetVolume`](SoundGetVolume.md)
