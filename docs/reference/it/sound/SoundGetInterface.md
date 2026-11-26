# `SoundGetInterface`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · function · Il suono

## Descrizione

Reads an interface pointer of a sound device (Windows WASAPI).

## Sintassi

```ahk
P := SoundGetInterface(Device, IID)
```

## Parametri

- **Device** — The device.
- **IID** — The interface id.

## Valore di ritorno

A pointer; refuses where there is no WASAPI.

## Esempio

```ahk
p := SoundGetInterface(1, "IAudioEndpointVolume")
```

## Correlati

[`SoundGetName`](SoundGetName.md), [`ComObject`](ComObject.md)
