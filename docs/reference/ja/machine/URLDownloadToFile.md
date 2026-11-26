# `URLDownloadToFile`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · マシン、プロセス、環境

## 説明

Downloads a URL to a file.

## 構文

```ahk
URLDownloadToFile URL, Filename
```

## パラメータ

- **URL** — The address.
- **Filename** — The destination.

## 戻り値

Nothing; throws on failure.

## 備考

On Linux curl; on Windows URLMon.

## 例

```ahk
URLDownloadToFile "https://x/a", "a"
```

## 関連

[`Download`](Download.md), [`FileRead`](FileRead.md)
