# `Integer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · class · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

Converts a value to an integer.

## 구문

```ahk
N := Integer(Value)
```

## 매개변수

- **Value** — A number or numeric text.

## 반환 값

The integer; throws TypeError if not convertible.

## 예제

```ahk
Integer("42")  ; 42
```

## 관련

[`Float`](Float.md), [`String`](String.md), [`Number`](Number.md)
