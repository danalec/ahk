# `EnvGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · パーサとスクリプトの実行

## 説明

Answers the value of an environment variable of the runner; the environment is the one of the process the script runs in.

## 構文

```ahk
V := EnvGet(Name)
```

## パラメータ

- **Name** — The variable.

## 戻り値

the value of the variable, or the empty text when the environment does not have it

## エラー

nothing; a name that is not there is the empty text and not a failure.

## 例

```ahk
home := EnvGet("HOME")
```

## 関連

[`EnvSet`](EnvSet.md)
