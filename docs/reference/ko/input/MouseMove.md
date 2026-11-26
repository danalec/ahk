# `MouseMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 핫키, 핫스트링, 입력

## 설명

Moves the pointer of the backend to a place on the screen. The box moves the pointer to the place, and not from where it is.

## 구문

```ahk
MouseMove X, Y, Speed
```

## 매개변수

- **X/Y** — Screen coords (or relative with R).
- **Speed** — 0..100.

## 반환 값

nothing

## 오류

the relative place of the move and the speed of the move are refused as not carried yet.

## 예제

```ahk
MouseMove 400, 300
```

## 관련

[`Click`](Click.md), [`MouseGetPos`](MouseGetPos.md)
