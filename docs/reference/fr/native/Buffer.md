# `Buffer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · class · Appels de fonctions natives (DllCall)

## Description

Allocates a block of bytes.

## Syntaxe

```ahk
Buf := Buffer(Size, Fill)
```

## Paramètres

- **Size** — Bytes.
- **Fill** — Initial byte value.

## Valeur de retour

A Buffer with Ptr/Size/__Item.

## Exemple

```ahk
b := Buffer(16, 0)
```

## Voir aussi

[`NumGet`](NumGet.md), [`NumPut`](NumPut.md), [`StrGet`](StrGet.md)
