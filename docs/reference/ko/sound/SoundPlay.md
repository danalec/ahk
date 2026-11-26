# `SoundPlay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 소리

## 설명

Plays a sound file with the mixer of the session; the volume, the speed and the pitch of v2 are not carried.

## 구문

```ahk
SoundPlay Filename, Wait
```

## 매개변수

- **Filename** — The file.
- **Wait** — 1 to wait.

## 반환 값

1

## 오류

the volume, the speed and the pitch are refused with "the volume, the speed and the pitch of a sound are not carried yet."; an empty path is "the sound file is missing."; a host without the mixer answers the reason of the player.

## 예제

```ahk
SoundPlay "/usr/share/sounds/freedesktop/stereo/complete.oga"
```

## 관련

[`SoundBeep`](SoundBeep.md), [`SoundSet`](SoundSet.md)
