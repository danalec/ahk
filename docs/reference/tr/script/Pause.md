# `Pause`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Ayrıştırıcı ve betiğin çalıştırılması

## Açıklama

Sets the paused state of the script. The tree keeps a state instead of waiting for a key, which would hang a script that has no keyboard. The state is visible in A_IsPaused.

## Sözdizimi

```ahk
Pause …
```

## Parametreler

[OnOff]

## Dönüş değeri

the empty text

## Hatalar

a parameter that is not 0, 1 or -1 is an invalid parameter.

## Örnek

```ahk
Pause true
```
