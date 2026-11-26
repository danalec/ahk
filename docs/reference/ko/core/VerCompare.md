# `VerCompare`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

Two version strings, compared. The numbers of the two versions are compared one by one, and the component that is missing is a zero. A version with a pre-release suffix is older than the same version without it.

## 구문

```ahk
VerCompare …
```

## 매개변수

Version1, Version2

## 반환 값

the order of the two versions as -1, 0 or 1; when the second version carries an operator -- <, <=, >, >= or = -- the answer is whether the relation holds

## 오류

none.

## 예제

```ahk
VerCompare("1.2.0", "1.10.0")
```
