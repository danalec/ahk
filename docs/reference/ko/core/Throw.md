# `Throw`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*KeySharp* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

A statement that raises a value. The value is usually an error object, and it may be any value.

## 구문

```ahk
Throw 
```

## 매개변수

Value

## 반환 값

nothing; the statement leaves the flow of the script

## 오류

none; the value is caught by the catch that names its class.

## 예제

```ahk
throw ValueError("boom")
```
