# `RegDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Interfejsy API rejestru

## Opis

Removes a value of the registry of the store of the runtime; the key itself stays.

## Składnia

```ahk
RegDelete …
```

## Parametry

KeyName, ValueName

## Wartość zwracana

the empty text

## Błędy

"Parameter #1 of RegDelete must not be omitted in this case." when the key is left out; a key without a hive of the ten is an invalid parameter; "(2) The system cannot find the file specified." when the key is not there.

## Przykład

```ahk
RegDelete "HKCU\Software\MyApp", "Setting"
```
