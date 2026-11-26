# `URLDownloadToFile`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 機器、行程與環境

## 描述

Downloads a URL to a file.

## 語法

```ahk
URLDownloadToFile URL, Filename
```

## 參數

- **URL** — The address.
- **Filename** — The destination.

## 傳回值

Nothing; throws on failure.

## 備註

On Linux curl; on Windows URLMon.

## 範例

```ahk
URLDownloadToFile "https://x/a", "a"
```

## 相關

[`Download`](Download.md), [`FileRead`](FileRead.md)
