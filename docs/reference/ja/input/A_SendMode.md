# `A_SendMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · variable · ホットキー、ホット文字列、入力

## 説明

The method that a `Send` uses: `Input`, `Event`, `Play` or `InputThenPlay`.

## 構文

```ahk
A_SendMode 
```

## パラメータ

nothing; it is a variable, not a call

## 戻り値

the name of the mode, as a text

## エラー

the write takes the name of one of the four modes, and any other name is refused with "Invalid value.".

## 例

```ahk
FileAppend A_SendMode, "*"
```
