# `SoundGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Звук

## Описание

Reads a sound component/control (v1 shape).

## Синтаксис

```ahk
SoundGet OutputVar, Component, Control, Device
```

## Параметры

- **OutputVar** — Receives the value.
- **Component/Control/Device** — The mixer selectors.

## Возвращаемое значение

Nothing; the var is filled.

## Замечания

On Linux pactl; the Windows mixer concepts map to volume/mute.

## Пример

```ahk
SoundGet &v, , "Volume"
```

## См. также

[`SoundSet`](SoundSet.md), [`SoundGetVolume`](SoundGetVolume.md)
