# `IniWrite`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Dosya ve dizin işlemleri

## Açıklama

Writes a key of an ini file. The value comes first and the name of the file second, the way the binary of v2.0.31 takes them; a file that is not there is made.

## Sözdizimi

```ahk
IniWrite Value, Filename, Section, Key
```

## Parametreler

- **Value** — The text.
- **Filename** — The INI.
- **Section** — The section.
- **Key** — The key.

## Dönüş değeri

nothing

## Hatalar

Error when the file cannot be written.

## Örnek

```ahk
IniWrite "hello", "app.ini", "Main", "Name"
```

## İlgili

[`IniRead`](IniRead.md), [`IniDelete`](IniDelete.md)
