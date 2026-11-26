# `SoundGetVolume`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · function · El sonido

## Descripción

Reads the volume of a device (0..100).

## Sintaxis

```ahk
V := SoundGetVolume(Device)
```

## Parámetros

- **Device** — The device; omitted default.

## Valor de retorno

The volume.

## Ejemplo

```ahk
v := SoundGetVolume()
```

## Relacionados

[`SoundSetVolume`](SoundSetVolume.md), [`SoundGetMute`](SoundGetMute.md)
