# `MouseClick`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 핫키, 핫스트링, 입력

## 설명

Clicks with the pointer of the backend. The button is the first parameter, the way the function of v2 takes it, and a button that is not there is the left one.

## 구문

```ahk
MouseClick …
```

## 매개변수

Button, X, Y [Count] [Options]

## 반환 값

nothing

## 오류

the wheel, the fourth and the fifth buttons are refused as not carried yet.

## 예제

```ahk
MouseClick "Right", 100, 200
```
