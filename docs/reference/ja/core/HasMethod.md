# `HasMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · method · `Object` · 言語の中核: テキスト、数値、正規表現

## 説明

Whether an object has a method.

## 構文

```ahk
HasMethod 
```

## パラメータ

the object, the name of the method, and the number of parameters it would take

## 戻り値

1 when the object has such a method, 0 when it has not

## エラー

nothing; a value that is not an object answers zero, and the question with no name asks whether the value is a class.

## 例

```ahk
o := {a: 1}
FileAppend HasMethod(o, "a"), "*"
```
