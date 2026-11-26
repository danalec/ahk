# `A_IsAdmin`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · variable · パーサとスクリプトの実行

## 説明

Whether the script runs with the rights of an administrator: the effective user id, and the root is the one that is zero.

## 構文

```ahk
A_IsAdmin 
```

## パラメータ

nothing; it is a variable, not a call

## 戻り値

1 when the script is elevated, 0 when it is not

## エラー

the read fails when the machine cannot say; the write is refused.

## 例

```ahk
FileAppend A_IsAdmin, "*"
```
