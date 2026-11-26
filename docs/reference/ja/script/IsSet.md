# `IsSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · パーサとスクリプトの実行

## 説明

Whether the variable of the caller has a value. The tree carries it besides `IsSetRef`, and it asks for the variable and not for its value.

## 構文

```ahk
IsSet …
```

## パラメータ

the variable, not its value

## 戻り値

1 when the variable has a value, 0 when it has none

## エラー

an argument that is not a variable is refused with "IsSet requires a variable.".

## 例

```ahk
x := 1
FileAppend IsSet(x), "*"
```
