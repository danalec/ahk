# `EnvSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 解析器與腳本執行

## 描述

Sets an environment variable of the runner and answers the value; the script runs on one thread, so nothing else reads the environment while the change happens.

## 語法

```ahk
EnvSet Name, Value
```

## 參數

- **Name** — The variable.
- **Value** — The value; omitted deletes.

## 傳回值

the value that was set, as text

## 錯誤

an empty name and a name that carries an equal sign are refused with the reason of the machine.

## 範例

```ahk
EnvSet "MY_VAR", "value"
```

## 相關

[`EnvGet`](EnvGet.md)
