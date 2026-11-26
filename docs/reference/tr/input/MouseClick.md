# `MouseClick`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Kısayollar, tetikleme dizeleri ve giriş

## Açıklama

Clicks with the pointer of the backend. The button is the first parameter, the way the function of v2 takes it, and a button that is not there is the left one.

## Sözdizimi

```ahk
MouseClick …
```

## Parametreler

Button, X, Y [Count] [Options]

## Dönüş değeri

nothing

## Hatalar

the wheel, the fourth and the fifth buttons are refused as not carried yet.

## Örnek

```ahk
MouseClick "Right", 100, 200
```
