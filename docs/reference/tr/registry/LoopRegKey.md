# `LoopRegKey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · variable · Kayıt defteri API'leri

## Açıklama

The name of the current subkey in a Loop Reg.

## Sözdizimi

```ahk
S := LoopRegKey
```

## Parametreler

?.

## Dönüş değeri

The subkey name.

## Örnek

```ahk
Loop Reg, "HKCU\Software"
    s := LoopRegKey
```

## İlgili

[`LoopRegName`](LoopRegName.md), [`LoopRegType`](LoopRegType.md)
