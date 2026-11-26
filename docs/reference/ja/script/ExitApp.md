# `ExitApp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · パーサとスクリプトの実行

## 説明

Ends the script with a code. The code goes into the exit_code of the report.

## 構文

```ahk
ExitApp ExitCode
```

## パラメータ

- **ExitCode** — The code the process exits with (default 0).

## 戻り値

the code

## エラー

none.

## 備考

OnExit callbacks run before the process ends.

## 例

```ahk
ExitApp 3
```

## 関連

[`Exit`](Exit.md), [`Reload`](Reload.md), [`OnExit`](OnExit.md)
