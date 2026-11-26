# `SoundBeep`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Ses

## Açıklama

Plays a tone; the box has no speaker of its own, so the tone is written as a wave file under /tmp, played with paplay, and removed again.

## Sözdizimi

```ahk
SoundBeep Frequency, Duration
```

## Parametreler

- **Frequency** — Hz.
- **Duration** — ms.

## Dönüş değeri

1

## Hatalar

a frequency outside the range is refused with "the frequency of a tone is between 37 and 32767."; a host without the mixer answers "the mixer refused: ... pactl: not found" and "the mixer could not play ...".

## Örnek

```ahk
SoundBeep 1000, 200
```

## İlgili

[`SoundPlay`](SoundPlay.md), [`SoundSet`](SoundSet.md)
