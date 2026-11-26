# `Object`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · 言語のオブジェクト: Array、Map、Func

## 説明

The root of the chain of the classes: every object of the runtime is one. `Object()` makes an object with nothing in it, and the literal `{a: 1}` makes one with a property.

## 構文

```ahk
Obj := Object(Props...)
```

## パラメータ

- **Props** — Alternating names and values.

## 戻り値

the object of the class `Object`

## エラー

nothing; a member that is not there is refused with the type of the value and the name of the member.

## 備考

Base is Object; OwnProps/HasProp/DefineProp/DeleteProp; __Get/__Set/__Call meta-functions.

## 例

```ahk
o := Object()
FileAppend o.__Class, "*"
```

## 関連

[`Map`](Map.md), [`Array`](Array.md), [`Gui`](Gui.md)
