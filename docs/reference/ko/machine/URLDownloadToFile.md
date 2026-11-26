# `URLDownloadToFile`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 머신, 프로세스, 환경

## 설명

Downloads a URL to a file.

## 구문

```ahk
URLDownloadToFile URL, Filename
```

## 매개변수

- **URL** — The address.
- **Filename** — The destination.

## 반환 값

Nothing; throws on failure.

## 설명

On Linux curl; on Windows URLMon.

## 예제

```ahk
URLDownloadToFile "https://x/a", "a"
```

## 관련

[`Download`](Download.md), [`FileRead`](FileRead.md)
