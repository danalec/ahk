# `HasBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · method · `Object` · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

Whether the base of an object is another object.

## 구문

```ahk
HasBase 
```

## 매개변수

the object, and the base

## 반환 값

1 when the base is that one, 0 when it is not

## 오류

a base that is not an object is refused before the question of the object is asked.

## 예제

```ahk
FileAppend HasBase({a: 1}, {}), "*"
```
