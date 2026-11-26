# `OwnProps`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · method · `Object` · 언어의 객체: Array, Map, Func

## 설명

Iterates the own properties of an object.

## 구문

```ahk
for name, value in Obj.OwnProps()
```

## 매개변수

?.

## 반환 값

The enumerator.

## 예제

```ahk
for n, v in o.OwnProps()
```

## 관련

[`HasOwnProp`](HasOwnProp.md), [`DefineProp`](DefineProp.md)
