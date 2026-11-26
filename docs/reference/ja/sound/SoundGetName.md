# `SoundGetName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · function · サウンド

## 説明

Reads the name of a sound device.

## 構文

```ahk
S := SoundGetName(Device)
```

## パラメータ

- **Device** — The device index.

## 戻り値

The name.

## 例

```ahk
n := SoundGetName()
```

## 関連

[`SoundGetInterface`](SoundGetInterface.md), [`SoundGetVolume`](SoundGetVolume.md)
