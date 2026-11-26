# `Ord`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · Der Kern der Sprache: Text, Mathematik und Muster

## Beschreibung

The code point of the first character of a text.

## Syntax

```ahk
N := Ord(String)
```

## Parameter

- **String** — The text.

## Rückgabewert

the code point, or zero when the text has no character

## Fehler

none; a lone surrogate answers with the code unit of it.

## Beispiel

```ahk
Ord("A")
```

## Verwandt

[`Chr`](Chr.md), [`Asc`](Asc.md)
