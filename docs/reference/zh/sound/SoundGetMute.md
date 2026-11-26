# `SoundGetMute`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · function · 声音

## 描述

Reads whether a device is muted.

## 语法

```ahk
B := SoundGetMute(Device)
```

## 参数

- **Device** — The device.

## 返回值

1/0.

## 示例

```ahk
m := SoundGetMute()
```

## 相关

[`SoundSetMute`](SoundSetMute.md), [`SoundGetVolume`](SoundGetVolume.md)
