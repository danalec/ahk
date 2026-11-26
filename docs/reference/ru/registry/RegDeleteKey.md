# `RegDeleteKey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · function · API реестра

## Описание

Deletes a registry key (Windows).

## Синтаксис

```ahk
RegDeleteKey KeyName
```

## Параметры

- **KeyName** — The key.

## Возвращаемое значение

Nothing.

## Пример

```ahk
RegDeleteKey "HKCU\Software\X"
```

## См. также

[`RegCreateKey`](RegCreateKey.md), [`RegDelete`](RegDelete.md)
