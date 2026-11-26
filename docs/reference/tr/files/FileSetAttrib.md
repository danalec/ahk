# `FileSetAttrib`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Dosya ve dizin işlemleri

## Açıklama

Changes the attributes of a file. The attributes come first and the name of the file second, the way the binary of v2.0.31 takes them. The name of the file may be a pattern, and the attributes are changed in every file that the pattern names.

## Sözdizimi

```ahk
FileSetAttrib …
```

## Parametreler

Attributes, FileName, [Recurse]

## Dönüş değeri

nothing

## Hatalar

ValueError when the attributes are not one of v2; OSError when the attributes of the file cannot be changed. The third parameter is the recursion: the "R" walks into the subfolders, and any other letter is refused.

## Örnek

```ahk
FileSetAttrib "+H", "*.txt"
```
