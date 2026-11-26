# `SoundGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Il suono

## Descrizione

Reads a sound component/control (v1 shape).

## Sintassi

```ahk
SoundGet OutputVar, Component, Control, Device
```

## Parametri

- **OutputVar** — Receives the value.
- **Component/Control/Device** — The mixer selectors.

## Valore di ritorno

Nothing; the var is filled.

## Osservazioni

On Linux pactl; the Windows mixer concepts map to volume/mute.

## Esempio

```ahk
SoundGet &v, , "Volume"
```

## Correlati

[`SoundSet`](SoundSet.md), [`SoundGetVolume`](SoundGetVolume.md)
