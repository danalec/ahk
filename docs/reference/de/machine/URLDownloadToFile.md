# `URLDownloadToFile`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Die Maschine, die Prozesse und die Umgebung

## Beschreibung

Downloads a URL to a file.

## Syntax

```ahk
URLDownloadToFile URL, Filename
```

## Parameter

- **URL** — The address.
- **Filename** — The destination.

## Rückgabewert

Nothing; throws on failure.

## Bemerkungen

On Linux curl; on Windows URLMon.

## Beispiel

```ahk
URLDownloadToFile "https://x/a", "a"
```

## Verwandt

[`Download`](Download.md), [`FileRead`](FileRead.md)
