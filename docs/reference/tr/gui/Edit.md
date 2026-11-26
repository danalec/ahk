# `Edit`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

Opens the file of the script in the program that the machine opens a file of that kind with -- the editor of a script.

## Sözdizimi

```ahk
Edit …
```

## Parametreler

none

## Dönüş değeri

the empty text

## Hatalar

the call fails when the script has no file: a script that comes from the standard input has none, and the reason is "the script of this runtime is read from the standard input, so it has no file for an editor to open".

## Örnek

```ahk
Edit()
```
