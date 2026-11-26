# `SoundGetVolume`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · function · Ses

## Açıklama

Reads the volume of a device (0..100).

## Sözdizimi

```ahk
V := SoundGetVolume(Device)
```

## Parametreler

- **Device** — The device; omitted default.

## Dönüş değeri

The volume.

## Örnek

```ahk
v := SoundGetVolume()
```

## İlgili

[`SoundSetVolume`](SoundSetVolume.md), [`SoundGetMute`](SoundGetMute.md)
