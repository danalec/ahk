# `LastError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · variable · Aufrufe nativer Funktionen (DllCall)

## Beschreibung

The last error code of the machine (the Win32 GetLastError).

## Syntax

```ahk
N := LastError
```

## Parameter

?.

## Rückgabewert

The code.

## Beispiel

```ahk
e := LastError
```

## Verwandt

[`DllCall`](DllCall.md), [`OSError`](OSError.md)
