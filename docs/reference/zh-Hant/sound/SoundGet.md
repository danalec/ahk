# `SoundGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 聲音

## 描述

Reads a sound component/control (v1 shape).

## 語法

```ahk
SoundGet OutputVar, Component, Control, Device
```

## 參數

- **OutputVar** — Receives the value.
- **Component/Control/Device** — The mixer selectors.

## 傳回值

Nothing; the var is filled.

## 備註

On Linux pactl; the Windows mixer concepts map to volume/mute.

## 範例

```ahk
SoundGet &v, , "Volume"
```

## 相關

[`SoundSet`](SoundSet.md), [`SoundGetVolume`](SoundGetVolume.md)
