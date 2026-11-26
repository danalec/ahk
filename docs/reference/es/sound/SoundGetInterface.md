# `SoundGetInterface`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · function · El sonido

## Descripción

Reads an interface pointer of a sound device (Windows WASAPI).

## Sintaxis

```ahk
P := SoundGetInterface(Device, IID)
```

## Parámetros

- **Device** — The device.
- **IID** — The interface id.

## Valor de retorno

A pointer; refuses where there is no WASAPI.

## Ejemplo

```ahk
p := SoundGetInterface(1, "IAudioEndpointVolume")
```

## Relacionados

[`SoundGetName`](SoundGetName.md), [`ComObject`](ComObject.md)
