# `SoundGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · サウンド

## 説明

Reads a sound component/control (v1 shape).

## 構文

```ahk
SoundGet OutputVar, Component, Control, Device
```

## パラメータ

- **OutputVar** — Receives the value.
- **Component/Control/Device** — The mixer selectors.

## 戻り値

Nothing; the var is filled.

## 備考

On Linux pactl; the Windows mixer concepts map to volume/mute.

## 例

```ahk
SoundGet &v, , "Volume"
```

## 関連

[`SoundSet`](SoundSet.md), [`SoundGetVolume`](SoundGetVolume.md)
