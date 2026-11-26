# `A_LoopReadLine`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · variable · ファイルとディレクトリの操作

## 説明

The line of the file the read loop is on.

## 構文

```ahk
A_LoopReadLine 
```

## パラメータ

nothing; it is a variable, not a call

## 戻り値

the line, as a text; the empty text before the first read loop

## エラー

nothing to read; the write is refused.

## 例

```ahk
Loop Read "notes.txt"
    FileAppend A_LoopReadLine, "*"
```
