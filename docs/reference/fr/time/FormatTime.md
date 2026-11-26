# `FormatTime`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Les dates et les heures

## Description

A moment written the way a format says. The date comes first and the format comes last; a text that carries no date at all is the date of the machine.

## Syntaxe

```ahk
S := FormatTime(YYYYMMDDHH24MISS, Format)
```

## Paramètres

- **Time** — A timestamp; omitted = now.
- **Format** — The picture (yyyy-MM-dd etc.).

## Valeur de retour

the date written the way the format says

## Erreurs

a date the dialect cannot read is an invalid parameter.

## Exemple

```ahk
FormatTime(A_Now, "yyyy-MM-dd")
```

## Voir aussi

[`DateAdd`](DateAdd.md), [`DateDiff`](DateDiff.md), [`A_Now`](A_Now.md)
