# `Reload`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · パーサとスクリプトの実行

## 説明

Starts the script again in a process of its own: the old instance ends and the new one carries on with the same file and the same arguments. A script that comes from the standard input has no file, and the v2 does nothing then.

## 構文

```ahk
Reload
```

## パラメータ

none

## 戻り値

the empty text

## エラー

none; the name takes no parameter, and a script without a file is not refused -- the call simply does nothing.

## 備考

The new instance replaces the old; single-instance rules apply.

## 例

```ahk
Reload
```

## 関連

[`ExitApp`](ExitApp.md), [`#SingleInstance`](#SingleInstance.md)
