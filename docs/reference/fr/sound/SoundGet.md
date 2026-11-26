# `SoundGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Le son

## Description

Reads a sound component/control (v1 shape).

## Syntaxe

```ahk
SoundGet OutputVar, Component, Control, Device
```

## Paramètres

- **OutputVar** — Receives the value.
- **Component/Control/Device** — The mixer selectors.

## Valeur de retour

Nothing; the var is filled.

## Remarques

On Linux pactl; the Windows mixer concepts map to volume/mute.

## Exemple

```ahk
SoundGet &v, , "Volume"
```

## Voir aussi

[`SoundSet`](SoundSet.md), [`SoundGetVolume`](SoundGetVolume.md)
