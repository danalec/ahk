# `HasBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · method · `Object` · Rdzeń języka: tekst, matematyka i wzorce

## Opis

Whether the base of an object is another object.

## Składnia

```ahk
HasBase 
```

## Parametry

the object, and the base

## Wartość zwracana

1 when the base is that one, 0 when it is not

## Błędy

a base that is not an object is refused before the question of the object is asked.

## Przykład

```ahk
FileAppend HasBase({a: 1}, {}), "*"
```
