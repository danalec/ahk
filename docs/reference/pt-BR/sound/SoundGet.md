# `SoundGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · O som

## Descrição

Reads a sound component/control (v1 shape).

## Sintaxe

```ahk
SoundGet OutputVar, Component, Control, Device
```

## Parâmetros

- **OutputVar** — Receives the value.
- **Component/Control/Device** — The mixer selectors.

## Valor de retorno

Nothing; the var is filled.

## Observações

On Linux pactl; the Windows mixer concepts map to volume/mute.

## Exemplo

```ahk
SoundGet &v, , "Volume"
```

## Relacionados

[`SoundSet`](SoundSet.md), [`SoundGetVolume`](SoundGetVolume.md)
