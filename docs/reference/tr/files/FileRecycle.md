# `FileRecycle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Dosya ve dizin işlemleri

## Açıklama

Sends a file, a folder, or the files that a pattern matches, to the trash of the machine. The box uses the trash of the XDG specification, and it is the home directory of the user that holds it.

## Sözdizimi

```ahk
FileRecycle …
```

## Parametreler

FileName

## Dönüş değeri

nothing

## Hatalar

Error `Failed` when the path is not there; a pattern that matches nothing is not a failure; ValueError when the name is empty; Error when the machine does not say where the home directory of the user is.

## Örnek

```ahk
FileRecycle "out.txt"
```
