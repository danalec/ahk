# `SoundPlay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · O som

## Descrição

Toca um arquivo de som com o mixer da sessão; o volume, a velocidade e o tom do v2 não são carregados.

## Sintaxe

```ahk
SoundPlay Filename, Wait
```

## Parâmetros

- **Filename** — The file.
- **Wait** — 1 to wait.

## Valor de retorno

1

## Erros

the volume, the speed and the pitch are refused with "the volume, the speed and the pitch of a sound are not carried yet."; an empty path is "the sound file is missing."; a host without the mixer answers the reason of the player.

## Exemplo

```ahk
SoundPlay "/usr/share/sounds/freedesktop/stereo/complete.oga"
```

## Relacionados

[`SoundBeep`](SoundBeep.md), [`SoundSet`](SoundSet.md)
