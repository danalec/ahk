# `GetMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · method · `Object` · 語言核心：文字、數學與模式

## 描述

The method of an object, as a value that can be called.

## 語法

```ahk
GetMethod 
```

## 參數

the object, the name of the method, and the number of parameters it would take

## 傳回值

the method, as a value of the class `Func`

## 錯誤

a method that is not there is refused with the type of the value and the name of the member.

## 範例

```ahk
o := {a: 1}
FileAppend GetMethod(o, "HasOwnProp").__Class, "*"
```
