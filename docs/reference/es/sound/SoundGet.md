# `SoundGet`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · El sonido

## Descripción

Reads a sound component/control (v1 shape).

## Sintaxis

```ahk
SoundGet OutputVar, Component, Control, Device
```

## Parámetros

- **OutputVar** — Receives the value.
- **Component/Control/Device** — The mixer selectors.

## Valor de retorno

Nothing; the var is filled.

## Observaciones

On Linux pactl; the Windows mixer concepts map to volume/mute.

## Ejemplo

```ahk
SoundGet &v, , "Volume"
```

## Relacionados

[`SoundSet`](SoundSet.md), [`SoundGetVolume`](SoundGetVolume.md)
