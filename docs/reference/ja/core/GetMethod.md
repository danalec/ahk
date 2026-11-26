# `GetMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · method · `Object` · 言語の中核: テキスト、数値、正規表現

## 説明

The method of an object, as a value that can be called.

## 構文

```ahk
GetMethod 
```

## パラメータ

the object, the name of the method, and the number of parameters it would take

## 戻り値

the method, as a value of the class `Func`

## エラー

a method that is not there is refused with the type of the value and the name of the member.

## 例

```ahk
o := {a: 1}
FileAppend GetMethod(o, "HasOwnProp").__Class, "*"
```
