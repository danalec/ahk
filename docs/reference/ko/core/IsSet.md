# `IsSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*KeySharp* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

Whether the variable of the caller has a value. The tree carries it besides `IsSetRef`, and it asks for the variable and not for its value.

## 구문

```ahk
IsSet 
```

## 매개변수

the variable, not its value

## 반환 값

1 when the variable has a value, 0 when it has none

## 오류

an argument that is not a variable is refused with "IsSet requires a variable.".

## 예제

```ahk
x := 1
FileAppend IsSet(x), "*"
```
