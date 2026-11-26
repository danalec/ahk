# `SoundPlay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Dźwięk

## Opis

Plays a sound file with the mixer of the session; the volume, the speed and the pitch of v2 are not carried.

## Składnia

```ahk
SoundPlay Filename, Wait
```

## Parametry

- **Filename** — The file.
- **Wait** — 1 to wait.

## Wartość zwracana

1

## Błędy

the volume, the speed and the pitch are refused with "the volume, the speed and the pitch of a sound are not carried yet."; an empty path is "the sound file is missing."; a host without the mixer answers the reason of the player.

## Przykład

```ahk
SoundPlay "/usr/share/sounds/freedesktop/stereo/complete.oga"
```

## Powiązane

[`SoundBeep`](SoundBeep.md), [`SoundSet`](SoundSet.md)
