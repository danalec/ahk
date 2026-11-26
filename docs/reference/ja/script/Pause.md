# `Pause`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · パーサとスクリプトの実行

## 説明

Sets the paused state of the script. The tree keeps a state instead of waiting for a key, which would hang a script that has no keyboard. The state is visible in A_IsPaused.

## 構文

```ahk
Pause …
```

## パラメータ

[OnOff]

## 戻り値

the empty text

## エラー

a parameter that is not 0, 1 or -1 is an invalid parameter.

## 例

```ahk
Pause true
```
