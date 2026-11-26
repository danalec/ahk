# `SoundGetInterface`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · function · Sound

## Description

Reads an interface pointer of a sound device (Windows WASAPI).

## Syntax

```ahk
P := SoundGetInterface(Device, IID)
```

## Parameters

- **Device** — The device.
- **IID** — The interface id.

## Return value

A pointer; refuses where there is no WASAPI.

## Example

```ahk
p := SoundGetInterface(1, "IAudioEndpointVolume")
```

## Related

[`SoundGetName`](SoundGetName.md), [`ComObject`](ComObject.md)
