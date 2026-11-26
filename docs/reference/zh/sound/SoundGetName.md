# `SoundGetName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · function · 声音

## 描述

Reads the name of a sound device.

## 语法

```ahk
S := SoundGetName(Device)
```

## 参数

- **Device** — The device index.

## 返回值

The name.

## 示例

```ahk
n := SoundGetName()
```

## 相关

[`SoundGetInterface`](SoundGetInterface.md), [`SoundGetVolume`](SoundGetVolume.md)
