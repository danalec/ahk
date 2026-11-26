# `EnvGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 解析器與腳本執行

## 描述

Answers the value of an environment variable of the runner; the environment is the one of the process the script runs in.

## 語法

```ahk
V := EnvGet(Name)
```

## 參數

- **Name** — The variable.

## 傳回值

the value of the variable, or the empty text when the environment does not have it

## 錯誤

nothing; a name that is not there is the empty text and not a failure.

## 範例

```ahk
home := EnvGet("HOME")
```

## 相關

[`EnvSet`](EnvSet.md)
