# `SoundPlay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Der Klang

## Beschreibung

Plays a sound file with the mixer of the session; the volume, the speed and the pitch of v2 are not carried.

## Syntax

```ahk
SoundPlay Filename, Wait
```

## Parameter

- **Filename** — The file.
- **Wait** — 1 to wait.

## Rückgabewert

1

## Fehler

the volume, the speed and the pitch are refused with "the volume, the speed and the pitch of a sound are not carried yet."; an empty path is "the sound file is missing."; a host without the mixer answers the reason of the player.

## Beispiel

```ahk
SoundPlay "/usr/share/sounds/freedesktop/stereo/complete.oga"
```

## Verwandt

[`SoundBeep`](SoundBeep.md), [`SoundSet`](SoundSet.md)
