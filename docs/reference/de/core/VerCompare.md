# `VerCompare`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · Der Kern der Sprache: Text, Mathematik und Muster

## Beschreibung

Two version strings, compared. The numbers of the two versions are compared one by one, and the component that is missing is a zero. A version with a pre-release suffix is older than the same version without it.

## Syntax

```ahk
VerCompare …
```

## Parameter

Version1, Version2

## Rückgabewert

the order of the two versions as -1, 0 or 1; when the second version carries an operator -- <, <=, >, >= or = -- the answer is whether the relation holds

## Fehler

none.

## Beispiel

```ahk
VerCompare("1.2.0", "1.10.0")
```
