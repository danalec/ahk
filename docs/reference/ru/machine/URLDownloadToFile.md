# `URLDownloadToFile`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Машина, процессы и окружение

## Описание

Downloads a URL to a file.

## Синтаксис

```ahk
URLDownloadToFile URL, Filename
```

## Параметры

- **URL** — The address.
- **Filename** — The destination.

## Возвращаемое значение

Nothing; throws on failure.

## Замечания

On Linux curl; on Windows URLMon.

## Пример

```ahk
URLDownloadToFile "https://x/a", "a"
```

## См. также

[`Download`](Download.md), [`FileRead`](FileRead.md)
