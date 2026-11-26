# `SoundGetInterface`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · function · 聲音

## 描述

Reads an interface pointer of a sound device (Windows WASAPI).

## 語法

```ahk
P := SoundGetInterface(Device, IID)
```

## 參數

- **Device** — The device.
- **IID** — The interface id.

## 傳回值

A pointer; refuses where there is no WASAPI.

## 範例

```ahk
p := SoundGetInterface(1, "IAudioEndpointVolume")
```

## 相關

[`SoundGetName`](SoundGetName.md), [`ComObject`](ComObject.md)
