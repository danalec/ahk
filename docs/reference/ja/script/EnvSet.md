# `EnvSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · パーサとスクリプトの実行

## 説明

Sets an environment variable of the runner and answers the value; the script runs on one thread, so nothing else reads the environment while the change happens.

## 構文

```ahk
EnvSet Name, Value
```

## パラメータ

- **Name** — The variable.
- **Value** — The value; omitted deletes.

## 戻り値

the value that was set, as text

## エラー

an empty name and a name that carries an equal sign are refused with the reason of the machine.

## 例

```ahk
EnvSet "MY_VAR", "value"
```

## 関連

[`EnvGet`](EnvGet.md)
