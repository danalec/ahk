# `Sleep`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · L'analyseur et l'exécution du script

## Description

Pauses the script for a number of milliseconds.

## Syntaxe

```ahk
Sleep Milliseconds
```

## Paramètres

- **Milliseconds** — 0..60000; a larger value is clamped to 60000, a negative one sleeps 0.

## Valeur de retour

the number of milliseconds waited

## Erreurs

none; a wait longer than 60000 milliseconds is cut at that number.

## Remarques

The pause is clamped, not refused. During the sleep the timers of the script may still run (the runtime checks them between statements).

## Exemple

```ahk
Sleep 500
```

## Voir aussi

[`SetTimer`](SetTimer.md), [`Critical`](Critical.md)
