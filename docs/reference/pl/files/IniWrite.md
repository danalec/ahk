# `IniWrite`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Operacje na plikach i katalogach

## Opis

Writes a key of an ini file. The value comes first and the name of the file second, the way the binary of v2.0.31 takes them; a file that is not there is made.

## Składnia

```ahk
IniWrite Value, Filename, Section, Key
```

## Parametry

- **Value** — The text.
- **Filename** — The INI.
- **Section** — The section.
- **Key** — The key.

## Wartość zwracana

nothing

## Błędy

Error when the file cannot be written.

## Przykład

```ahk
IniWrite "hello", "app.ini", "Main", "Name"
```

## Powiązane

[`IniRead`](IniRead.md), [`IniDelete`](IniDelete.md)
