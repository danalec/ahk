# `SoundGetVolume`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · function · Звук

## Описание

Reads the volume of a device (0..100).

## Синтаксис

```ahk
V := SoundGetVolume(Device)
```

## Параметры

- **Device** — The device; omitted default.

## Возвращаемое значение

The volume.

## Пример

```ahk
v := SoundGetVolume()
```

## См. также

[`SoundSetVolume`](SoundSetVolume.md), [`SoundGetMute`](SoundGetMute.md)
