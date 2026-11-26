# `SoundGetInterface`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · function · Dźwięk

## Opis

Reads an interface pointer of a sound device (Windows WASAPI).

## Składnia

```ahk
P := SoundGetInterface(Device, IID)
```

## Parametry

- **Device** — The device.
- **IID** — The interface id.

## Wartość zwracana

A pointer; refuses where there is no WASAPI.

## Przykład

```ahk
p := SoundGetInterface(1, "IAudioEndpointVolume")
```

## Powiązane

[`SoundGetName`](SoundGetName.md), [`ComObject`](ComObject.md)
