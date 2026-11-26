# `SoundPlay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Ses

## Açıklama

Plays a sound file with the mixer of the session; the volume, the speed and the pitch of v2 are not carried.

## Sözdizimi

```ahk
SoundPlay Filename, Wait
```

## Parametreler

- **Filename** — The file.
- **Wait** — 1 to wait.

## Dönüş değeri

1

## Hatalar

the volume, the speed and the pitch are refused with "the volume, the speed and the pitch of a sound are not carried yet."; an empty path is "the sound file is missing."; a host without the mixer answers the reason of the player.

## Örnek

```ahk
SoundPlay "/usr/share/sounds/freedesktop/stereo/complete.oga"
```

## İlgili

[`SoundBeep`](SoundBeep.md), [`SoundSet`](SoundSet.md)
