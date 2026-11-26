# `SendLevel`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Kısayollar, tetikleme dizeleri ve giriş

## Açıklama

Carries the level of the events the script sends, and answers the level that was there before. The level tells the hotkeys which of the artificial events they are to ignore, and nothing reads it yet.

## Sözdizimi

```ahk
SendLevel …
```

## Parametreler

Level

## Dönüş değeri

the level that was there before, a number

## Hatalar

a level outside 0 to 100 is refused as an invalid parameter.

## Örnek

```ahk
SendLevel 1
```
