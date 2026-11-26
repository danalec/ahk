# `SoundGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Ses

## Açıklama

Reads a sound component/control (v1 shape).

## Sözdizimi

```ahk
SoundGet OutputVar, Component, Control, Device
```

## Parametreler

- **OutputVar** — Receives the value.
- **Component/Control/Device** — The mixer selectors.

## Dönüş değeri

Nothing; the var is filled.

## Notlar

On Linux pactl; the Windows mixer concepts map to volume/mute.

## Örnek

```ahk
SoundGet &v, , "Volume"
```

## İlgili

[`SoundSet`](SoundSet.md), [`SoundGetVolume`](SoundGetVolume.md)
