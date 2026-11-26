# `LoopRegTimeModified`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · variable · Les API du registre

## Description

The last-write time of the current registry key.

## Syntaxe

```ahk
S := LoopRegTimeModified
```

## Paramètres

?.

## Valeur de retour

A YYYYMMDDHH24MISS timestamp.

## Exemple

```ahk
t := LoopRegTimeModified
```

## Voir aussi

[`LoopRegKey`](LoopRegKey.md), [`FormatTime`](FormatTime.md)
