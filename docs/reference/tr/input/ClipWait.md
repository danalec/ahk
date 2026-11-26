# `ClipWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Kısayollar, tetikleme dizeleri ve giriş

## Açıklama

Waits for the clipboard to hold data.

## Sözdizimi

```ahk
ClipWait Timeout, WaitForAny
```

## Parametreler

0..255.

## Dönüş değeri

Nothing; throws TimeoutError.

## Örnek

```ahk
ClipWait 2
```

## İlgili

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
