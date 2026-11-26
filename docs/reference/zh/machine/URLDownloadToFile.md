# `URLDownloadToFile`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 机器、进程与环境

## 描述

Downloads a URL to a file.

## 语法

```ahk
URLDownloadToFile URL, Filename
```

## 参数

- **URL** — The address.
- **Filename** — The destination.

## 返回值

Nothing; throws on failure.

## 备注

On Linux curl; on Windows URLMon.

## 示例

```ahk
URLDownloadToFile "https://x/a", "a"
```

## 相关

[`Download`](Download.md), [`FileRead`](FileRead.md)
