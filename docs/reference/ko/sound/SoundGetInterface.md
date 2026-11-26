# `SoundGetInterface`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · function · 소리

## 설명

Reads an interface pointer of a sound device (Windows WASAPI).

## 구문

```ahk
P := SoundGetInterface(Device, IID)
```

## 매개변수

- **Device** — The device.
- **IID** — The interface id.

## 반환 값

A pointer; refuses where there is no WASAPI.

## 예제

```ahk
p := SoundGetInterface(1, "IAudioEndpointVolume")
```

## 관련

[`SoundGetName`](SoundGetName.md), [`ComObject`](ComObject.md)
