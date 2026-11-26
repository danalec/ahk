# `SoundGetVolume`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · function · 소리

## 설명

Reads the volume of a device (0..100).

## 구문

```ahk
V := SoundGetVolume(Device)
```

## 매개변수

- **Device** — The device; omitted default.

## 반환 값

The volume.

## 예제

```ahk
v := SoundGetVolume()
```

## 관련

[`SoundSetVolume`](SoundSetVolume.md), [`SoundGetMute`](SoundGetMute.md)
