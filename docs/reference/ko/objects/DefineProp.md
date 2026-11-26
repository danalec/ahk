# `DefineProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · method · `Object` · 언어의 객체: Array, Map, Func

## 설명

Defines a property with a getter/setter or value.

## 구문

```ahk
Obj.DefineProp(Name, Desc)
```

## 매개변수

- **Name** — The property.
- **Desc** — An object with Value/Get/Set/Call.

## 반환 값

Nothing.

## 예제

```ahk
o.DefineProp("x", {Get: (*) => 1})
```

## 관련

[`DeleteProp`](DeleteProp.md), [`HasProp`](HasProp.md), [`OwnProps`](OwnProps.md)
