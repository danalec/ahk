# `SoundGet`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · Sound

## Description

Reads a sound component/control (v1 shape).

## Syntax

```ahk
SoundGet OutputVar, Component, Control, Device
```

## Parameters

- **OutputVar** — Receives the value.
- **Component/Control/Device** — The mixer selectors.

## Return value

Nothing; the var is filled.

## Remarks

On Linux pactl; the Windows mixer concepts map to volume/mute.

## Example

```ahk
SoundGet &v, , "Volume"
```

## Related

[`SoundSet`](SoundSet.md), [`SoundGetVolume`](SoundGetVolume.md)
