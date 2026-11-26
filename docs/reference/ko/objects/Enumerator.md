# `Enumerator`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · class · 언어의 객체: Array, Map, Func

## 설명

The class of the walker of a loop: an enumerator hands out the pieces of a sequence, one turn at a time. The class is not made; the `__Enum` of a value makes one, and the call of it is the turn of the enumeration.

## 구문

```ahk
Enumerator 
```

## 매개변수

nothing that makes it

## 반환 값

the class of an enumerator, as a text

## 오류

the call of the class is refused with "Invalid base.".

## 예제

```ahk
for item in [1, 2]
    FileAppend Type(item), "*"
```
