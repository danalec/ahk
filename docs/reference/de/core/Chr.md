# `Chr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · Der Kern der Sprache: Text, Mathematik und Muster

## Beschreibung

The character with a code point.

## Syntax

```ahk
S := Chr(Number)
```

## Parameter

- **Number** — The code point.

## Rückgabewert

the text of that character

## Fehler

a code outside 0 to 0x10FFFF is an invalid parameter; a surrogate code is a text of its own.

## Beispiel

```ahk
Chr(65)
```

## Verwandt

[`Ord`](Ord.md), [`Asc`](Asc.md)
