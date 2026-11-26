# `Sleep`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · パーサとスクリプトの実行

## 説明

Pauses the script for a number of milliseconds.

## 構文

```ahk
Sleep Milliseconds
```

## パラメータ

- **Milliseconds** — 0..60000; a larger value is clamped to 60000, a negative one sleeps 0.

## 戻り値

the number of milliseconds waited

## エラー

none; a wait longer than 60000 milliseconds is cut at that number.

## 備考

The pause is clamped, not refused. During the sleep the timers of the script may still run (the runtime checks them between statements).

## 例

```ahk
Sleep 500
```

## 関連

[`SetTimer`](SetTimer.md), [`Critical`](Critical.md)
