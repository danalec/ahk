# `SoundGetMute`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · function · 소리

## 설명

Reads whether a device is muted.

## 구문

```ahk
B := SoundGetMute(Device)
```

## 매개변수

- **Device** — The device.

## 반환 값

1/0.

## 예제

```ahk
m := SoundGetMute()
```

## 관련

[`SoundSetMute`](SoundSetMute.md), [`SoundGetVolume`](SoundGetVolume.md)
