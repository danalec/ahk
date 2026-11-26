# `IniDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Dosya ve dizin işlemleri

## Açıklama

Takes a key of an ini file away. A key that is not given takes the whole section with it.

## Sözdizimi

```ahk
IniDelete …
```

## Parametreler

FileName, Section, [Key]

## Dönüş değeri

nothing

## Hatalar

Error when the file cannot be written.

## Örnek

```ahk
IniDelete "app.ini", "Main", "Name"
```
