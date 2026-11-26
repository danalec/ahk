# `GetOwnPropDesc`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · method · `Object` · 언어의 객체: Array, Map, Func

## 설명

The descriptor of an own property.

## 구문

```ahk
D := Obj.GetOwnPropDesc(Name)
```

## 매개변수

?.

## 반환 값

The descriptor object.

## 예제

```ahk
o.GetOwnPropDesc("x")
```

## 관련

[`DefineProp`](DefineProp.md), [`OwnProps`](OwnProps.md)
