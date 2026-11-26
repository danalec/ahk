# `RegCreateKey`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · function · Registry APIs

## Description

Creates a registry key (Windows).

## Syntax

```ahk
RegCreateKey KeyName
```

## Parameters

- **KeyName** — The key.

## Return value

Nothing.

## Example

```ahk
RegCreateKey "HKCU\Software\X"
```

## Related

[`RegDeleteKey`](RegDeleteKey.md), [`RegWrite`](RegWrite.md)
