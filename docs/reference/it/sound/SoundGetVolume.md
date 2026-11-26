# `SoundGetVolume`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · function · Il suono

## Descrizione

Reads the volume of a device (0..100).

## Sintassi

```ahk
V := SoundGetVolume(Device)
```

## Parametri

- **Device** — The device; omitted default.

## Valore di ritorno

The volume.

## Esempio

```ahk
v := SoundGetVolume()
```

## Correlati

[`SoundSetVolume`](SoundSetVolume.md), [`SoundGetMute`](SoundGetMute.md)
