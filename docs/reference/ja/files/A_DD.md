# `A_DD`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · variable · ファイルとディレクトリの操作

## 説明

The day of the month of the local time, in two digits.

## 構文

```ahk
A_DD 
```

## パラメータ

nothing; it is a variable, not a call

## 戻り値

the day, as a text of two digits

## エラー

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## 例

```ahk
FileAppend A_DD, "*"
```
