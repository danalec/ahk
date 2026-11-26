# `Throw`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*KeySharp* · function · Der Kern der Sprache: Text, Mathematik und Muster

## Beschreibung

A statement that raises a value. The value is usually an error object, and it may be any value.

## Syntax

```ahk
Throw 
```

## Parameter

Value

## Rückgabewert

nothing; the statement leaves the flow of the script

## Fehler

none; the value is caught by the catch that names its class.

## Beispiel

```ahk
throw ValueError("boom")
```
