# `GetMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · method · `Object` · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

The method of an object, as a value that can be called.

## 구문

```ahk
GetMethod 
```

## 매개변수

the object, the name of the method, and the number of parameters it would take

## 반환 값

the method, as a value of the class `Func`

## 오류

a method that is not there is refused with the type of the value and the name of the member.

## 예제

```ahk
o := {a: 1}
FileAppend GetMethod(o, "HasOwnProp").__Class, "*"
```
