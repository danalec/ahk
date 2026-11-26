# `Edit`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Okna GUI i automatyzacja okien

## Opis

Opens the file of the script in the program that the machine opens a file of that kind with -- the editor of a script.

## Składnia

```ahk
Edit …
```

## Parametry

none

## Wartość zwracana

the empty text

## Błędy

the call fails when the script has no file: a script that comes from the standard input has none, and the reason is "the script of this runtime is read from the standard input, so it has no file for an editor to open".

## Przykład

```ahk
Edit()
```
