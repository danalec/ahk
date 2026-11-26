# `LastError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · variable · ネイティブ関数の呼び出し (DllCall)

## 説明

The last error code of the machine (the Win32 GetLastError).

## 構文

```ahk
N := LastError
```

## パラメータ

?.

## 戻り値

The code.

## 例

```ahk
e := LastError
```

## 関連

[`DllCall`](DllCall.md), [`OSError`](OSError.md)
