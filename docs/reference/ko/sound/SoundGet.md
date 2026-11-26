# `SoundGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 소리

## 설명

Reads a sound component/control (v1 shape).

## 구문

```ahk
SoundGet OutputVar, Component, Control, Device
```

## 매개변수

- **OutputVar** — Receives the value.
- **Component/Control/Device** — The mixer selectors.

## 반환 값

Nothing; the var is filled.

## 설명

On Linux pactl; the Windows mixer concepts map to volume/mute.

## 예제

```ahk
SoundGet &v, , "Volume"
```

## 관련

[`SoundSet`](SoundSet.md), [`SoundGetVolume`](SoundGetVolume.md)
