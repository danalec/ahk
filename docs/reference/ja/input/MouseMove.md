# `MouseMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · ホットキー、ホット文字列、入力

## 説明

Moves the pointer of the backend to a place on the screen. The box moves the pointer to the place, and not from where it is.

## 構文

```ahk
MouseMove X, Y, Speed
```

## パラメータ

- **X/Y** — Screen coords (or relative with R).
- **Speed** — 0..100.

## 戻り値

nothing

## エラー

the relative place of the move and the speed of the move are refused as not carried yet.

## 例

```ahk
MouseMove 400, 300
```

## 関連

[`Click`](Click.md), [`MouseGetPos`](MouseGetPos.md)
