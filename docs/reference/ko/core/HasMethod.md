# `HasMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · method · `Object` · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

Whether an object has a method.

## 구문

```ahk
HasMethod 
```

## 매개변수

the object, the name of the method, and the number of parameters it would take

## 반환 값

1 when the object has such a method, 0 when it has not

## 오류

nothing; a value that is not an object answers zero, and the question with no name asks whether the value is a class.

## 예제

```ahk
o := {a: 1}
FileAppend HasMethod(o, "a"), "*"
```
