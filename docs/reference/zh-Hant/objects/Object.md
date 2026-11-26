# `Object`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 語言的物件：Array、Map 與 Func

## 描述

The root of the chain of the classes: every object of the runtime is one. `Object()` makes an object with nothing in it, and the literal `{a: 1}` makes one with a property.

## 語法

```ahk
Obj := Object(Props...)
```

## 參數

- **Props** — Alternating names and values.

## 傳回值

the object of the class `Object`

## 錯誤

nothing; a member that is not there is refused with the type of the value and the name of the member.

## 備註

Base is Object; OwnProps/HasProp/DefineProp/DeleteProp; __Get/__Set/__Call meta-functions.

## 範例

```ahk
o := Object()
FileAppend o.__Class, "*"
```

## 相關

[`Map`](Map.md), [`Array`](Array.md), [`Gui`](Gui.md)
