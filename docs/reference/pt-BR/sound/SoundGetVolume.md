# `SoundGetVolume`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · function · O som

## Descrição

Reads the volume of a device (0..100).

## Sintaxe

```ahk
V := SoundGetVolume(Device)
```

## Parâmetros

- **Device** — The device; omitted default.

## Valor de retorno

The volume.

## Exemplo

```ahk
v := SoundGetVolume()
```

## Relacionados

[`SoundSetVolume`](SoundSetVolume.md), [`SoundGetMute`](SoundGetMute.md)
