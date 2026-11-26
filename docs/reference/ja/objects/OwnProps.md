# `OwnProps`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · method · `Object` · 言語のオブジェクト: Array、Map、Func

## 説明

Iterates the own properties of an object.

## 構文

```ahk
for name, value in Obj.OwnProps()
```

## パラメータ

?.

## 戻り値

The enumerator.

## 例

```ahk
for n, v in o.OwnProps()
```

## 関連

[`HasOwnProp`](HasOwnProp.md), [`DefineProp`](DefineProp.md)
