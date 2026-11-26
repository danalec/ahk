# `Func`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · 言語のオブジェクト: Array、Map、Func

## 説明

The class of a function: a function of the script, a method, a lambda or a built-in. The class is not made: `Func(...)` answers "Invalid base."; the class of a function value is `Func`.

## 構文

```ahk
Func …
```

## パラメータ

nothing that makes it

## 戻り値

the class of a function value, as a text

## エラー

the call of the class is refused with "Invalid base.".

## 例

```ahk
FileAppend Abs.__Class, "*"
```
