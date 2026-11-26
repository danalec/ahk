# `SendMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Kısayollar, tetikleme dizeleri ve giriş

## Açıklama

Says how the keys are sent. The keys of this runtime go through the keyboard of the box, one at a time, and that is the mode of the box; the other modes of v2 are the ones of Windows.

## Sözdizimi

```ahk
SendMode …
```

## Parametreler

Method

## Dönüş değeri

the number 1 for the mode of the box

## Hatalar

a mode that is not 'Input' fails with 'the keys of this runtime go through the keyboard of the box, and that is the only mode there is.'

## Örnek

```ahk
SendMode "Input"
```
