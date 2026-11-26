# `SoundGetInterface`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · function · Der Klang

## Beschreibung

Reads an interface pointer of a sound device (Windows WASAPI).

## Syntax

```ahk
P := SoundGetInterface(Device, IID)
```

## Parameter

- **Device** — The device.
- **IID** — The interface id.

## Rückgabewert

A pointer; refuses where there is no WASAPI.

## Beispiel

```ahk
p := SoundGetInterface(1, "IAudioEndpointVolume")
```

## Verwandt

[`SoundGetName`](SoundGetName.md), [`ComObject`](ComObject.md)
