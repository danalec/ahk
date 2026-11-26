# `A_KeyDelay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · variable · ホットキー、ホット文字列、入力

## 説明

The delay between the keystrokes of a `Send`, in milliseconds. It is a setting of the run, and it is ten by default.

## 構文

```ahk
A_KeyDelay 
```

## パラメータ

nothing; it is a variable, not a call

## 戻り値

the delay, as a number

## エラー

the write takes the number of the value, and a text that holds no number is zero.

## 例

```ahk
FileAppend A_KeyDelay, "*"
```
