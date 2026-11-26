# `RegDeleteKey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · function · Interfejsy API rejestru

## Opis

Deletes a registry key (Windows).

## Składnia

```ahk
RegDeleteKey KeyName
```

## Parametry

- **KeyName** — The key.

## Wartość zwracana

Nothing.

## Przykład

```ahk
RegDeleteKey "HKCU\Software\X"
```

## Powiązane

[`RegCreateKey`](RegCreateKey.md), [`RegDelete`](RegDelete.md)
