# `Random`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Der Kern der Sprache: Text, Mathematik und Muster

## Beschreibung

A number between the two the caller names. Without arguments it is a number between zero and one; with one it is a whole number below it; with two it is a whole number between them, both ends counted.

## Syntax

```ahk
N := Random(Min, Max)
```

## Parameter

- **Min/Max** — The range; omitted = float 0..1.
- **Seed** — With a single arg, reseeds.

## Rückgabewert

the number

## Fehler

none.

## Beispiel

```ahk
Random(1, 6)
```

## Verwandt

[`Sort`](Sort.md)
