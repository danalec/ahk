# `Throw`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*KeySharp* · function · Rdzeń języka: tekst, matematyka i wzorce

## Opis

A statement that raises a value. The value is usually an error object, and it may be any value.

## Składnia

```ahk
Throw 
```

## Parametry

Value

## Wartość zwracana

nothing; the statement leaves the flow of the script

## Błędy

none; the value is caught by the catch that names its class.

## Przykład

```ahk
throw ValueError("boom")
```
