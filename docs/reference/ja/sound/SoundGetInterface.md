# `SoundGetInterface`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · function · サウンド

## 説明

Reads an interface pointer of a sound device (Windows WASAPI).

## 構文

```ahk
P := SoundGetInterface(Device, IID)
```

## パラメータ

- **Device** — The device.
- **IID** — The interface id.

## 戻り値

A pointer; refuses where there is no WASAPI.

## 例

```ahk
p := SoundGetInterface(1, "IAudioEndpointVolume")
```

## 関連

[`SoundGetName`](SoundGetName.md), [`ComObject`](ComObject.md)
