# `SoundGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 声音

## 描述

Reads a sound component/control (v1 shape).

## 语法

```ahk
SoundGet OutputVar, Component, Control, Device
```

## 参数

- **OutputVar** — Receives the value.
- **Component/Control/Device** — The mixer selectors.

## 返回值

Nothing; the var is filled.

## 备注

On Linux pactl; the Windows mixer concepts map to volume/mute.

## 示例

```ahk
SoundGet &v, , "Volume"
```

## 相关

[`SoundSet`](SoundSet.md), [`SoundGetVolume`](SoundGetVolume.md)
