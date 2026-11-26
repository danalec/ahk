# `SoundBeep`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · O som

## Descrição

Toca um tom; a caixa não tem alto-falante próprio, então o tom vira um arquivo wave em /tmp, é tocado com paplay e depois removido.

## Sintaxe

```ahk
SoundBeep Frequency, Duration
```

## Parâmetros

- **Frequency** — Hz.
- **Duration** — ms.

## Valor de retorno

1

## Erros

a frequency outside the range is refused with "the frequency of a tone is between 37 and 32767."; a host without the mixer answers "the mixer refused: ... pactl: not found" and "the mixer could not play ...".

## Exemplo

```ahk
SoundBeep 1000, 200
```

## Relacionados

[`SoundPlay`](SoundPlay.md), [`SoundSet`](SoundSet.md)
