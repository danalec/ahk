# `SoundPlay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 声音

## 描述

用会话的混音器播放一个声音文件；v2 的音量、速度和音高没有被承载。

## 语法

```ahk
SoundPlay Filename, Wait
```

## 参数

- **Filename** — The file.
- **Wait** — 1 to wait.

## 返回值

1

## 错误

the volume, the speed and the pitch are refused with "the volume, the speed and the pitch of a sound are not carried yet."; an empty path is "the sound file is missing."; a host without the mixer answers the reason of the player.

## 示例

```ahk
SoundPlay "/usr/share/sounds/freedesktop/stereo/complete.oga"
```

## 相关

[`SoundBeep`](SoundBeep.md), [`SoundSet`](SoundSet.md)
