# `A_Desktop`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · variable · マシン、プロセス、環境

## 説明

The folder of the desktop of the user: the folder the machine knows, and `$HOME/Desktop` when it knows none.

## 構文

```ahk
A_Desktop 
```

## パラメータ

nothing; it is a variable, not a call

## 戻り値

the path of the folder, as a text

## エラー

the read fails with "the machine has no such folder." when the machine has no such folder; the write is refused.

## 例

```ahk
FileAppend A_Desktop, "*"
```
