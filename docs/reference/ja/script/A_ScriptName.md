# `A_ScriptName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · variable · パーサとスクリプトの実行

## 説明

The name of the script. The script of the runner comes on the standard input and has no file, so the name is an asterisk. A script may write the name, and the value written is the one it keeps.

## 構文

```ahk
A_ScriptName 
```

## パラメータ

nothing; it is a variable, not a call

## 戻り値

the text "*", or the text that was written

## エラー

nothing to read; the write takes any value.

## 例

```ahk
FileAppend A_ScriptName, "*"
```
