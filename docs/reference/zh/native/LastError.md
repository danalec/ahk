# `LastError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · variable · 原生函数调用 (DllCall)

## 描述

The last error code of the machine (the Win32 GetLastError).

## 语法

```ahk
N := LastError
```

## 参数

?.

## 返回值

The code.

## 示例

```ahk
e := LastError
```

## 相关

[`DllCall`](DllCall.md), [`OSError`](OSError.md)
