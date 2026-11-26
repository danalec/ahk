# `SoundGetWaveVolume`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 소리

## 설명

Reads the wave volume (v1).

## 구문

```ahk
V := SoundGetWaveVolume(Device)
```

## 매개변수

- **Device** — The device.

## 반환 값

The volume.

## 예제

```ahk
v := SoundGetWaveVolume()
```

## 관련

[`SoundSetWaveVolume`](SoundSetWaveVolume.md), [`SoundGetVolume`](SoundGetVolume.md)
