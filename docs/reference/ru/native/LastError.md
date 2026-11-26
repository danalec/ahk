# `LastError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · variable · Вызовы нативных функций (DllCall)

## Описание

The last error code of the machine (the Win32 GetLastError).

## Синтаксис

```ahk
N := LastError
```

## Параметры

?.

## Возвращаемое значение

The code.

## Пример

```ahk
e := LastError
```

## См. также

[`DllCall`](DllCall.md), [`OSError`](OSError.md)
