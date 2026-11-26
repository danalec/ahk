# `FileSetTime`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Operacje na plikach i katalogach

## Opis

Changes the time of a file. The time comes first and the name of the file second, the way the binary of v2.0.31 takes them; a time that is blank is the moment of the machine. The name of the file may be a pattern, and the time is changed in every file that the pattern names.

## Składnia

```ahk
FileSetTime …
```

## Parametry

TimeStamp, FileName, [Which], [Recurse]

## Wartość zwracana

nothing

## Błędy

ValueError when the stamp is not one of v2, or when the time that is wanted is not the "M", the "C" or the "A"; OSError when the times of the file cannot be changed. The fourth parameter is the recursion: the "R" walks into the subfolders, and any other letter is refused.

## Przykład

```ahk
FileSetTime "20000101000000", "out.txt", "M"
```
