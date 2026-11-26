# `ObjGetBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · function · 언어의 객체: Array, Map, Func

## 설명

The base of an object: the prototype of its class.

## 구문

```ahk
ObjGetBase …
```

## 매개변수

the object

## 반환 값

the object of the base

## 오류

a value that is not an object is refused with "Expected an object but got ...".

## 예제

```ahk
FileAppend ObjGetBase({a: 1}).__Class, "*"
```
