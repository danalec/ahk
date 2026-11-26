# `A_LoopRegName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · variable · レジストリの API

## 説明

The name of the registry key the registry loop is on. The registry is a thing of Windows and this runtime has none: the registry loop fails with "the registry is a Windows thing and this box has none.", and the name keeps the empty text.

## 構文

```ahk
A_LoopRegName 
```

## パラメータ

nothing; it is a variable, not a call

## 戻り値

the empty text; the name is never written by the tree

## エラー

nothing to read; the write is refused.

## 例

```ahk
FileAppend A_LoopRegName, "*"
```
