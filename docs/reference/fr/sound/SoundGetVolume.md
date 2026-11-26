# `SoundGetVolume`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · function · Le son

## Description

Reads the volume of a device (0..100).

## Syntaxe

```ahk
V := SoundGetVolume(Device)
```

## Paramètres

- **Device** — The device; omitted default.

## Valeur de retour

The volume.

## Exemple

```ahk
v := SoundGetVolume()
```

## Voir aussi

[`SoundSetVolume`](SoundSetVolume.md), [`SoundGetMute`](SoundGetMute.md)
