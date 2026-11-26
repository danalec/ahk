# `LastError`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · variable · Llamadas a funciones nativas (DllCall)

## Descripción

The last error code of the machine (the Win32 GetLastError).

## Sintaxis

```ahk
N := LastError
```

## Parámetros

?.

## Valor de retorno

The code.

## Ejemplo

```ahk
e := LastError
```

## Relacionados

[`DllCall`](DllCall.md), [`OSError`](OSError.md)
