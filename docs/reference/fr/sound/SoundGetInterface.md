# `SoundGetInterface`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · function · Le son

## Description

Reads an interface pointer of a sound device (Windows WASAPI).

## Syntaxe

```ahk
P := SoundGetInterface(Device, IID)
```

## Paramètres

- **Device** — The device.
- **IID** — The interface id.

## Valeur de retour

A pointer; refuses where there is no WASAPI.

## Exemple

```ahk
p := SoundGetInterface(1, "IAudioEndpointVolume")
```

## Voir aussi

[`SoundGetName`](SoundGetName.md), [`ComObject`](ComObject.md)
