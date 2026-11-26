# `SoundBeep`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 声音

## 描述

播放一个音调；盒子自己没有扬声器，所以音调会写成 /tmp 下的波形文件，用 paplay 播放，然后删掉。

## 语法

```ahk
SoundBeep Frequency, Duration
```

## 参数

- **Frequency** — Hz.
- **Duration** — ms.

## 返回值

1

## 错误

a frequency outside the range is refused with "the frequency of a tone is between 37 and 32767."; a host without the mixer answers "the mixer refused: ... pactl: not found" and "the mixer could not play ...".

## 示例

```ahk
SoundBeep 1000, 200
```

## 相关

[`SoundPlay`](SoundPlay.md), [`SoundSet`](SoundSet.md)
