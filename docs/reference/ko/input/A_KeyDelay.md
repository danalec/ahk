# `A_KeyDelay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · variable · 핫키, 핫스트링, 입력

## 설명

The delay between the keystrokes of a `Send`, in milliseconds. It is a setting of the run, and it is ten by default.

## 구문

```ahk
A_KeyDelay 
```

## 매개변수

nothing; it is a variable, not a call

## 반환 값

the delay, as a number

## 오류

the write takes the number of the value, and a text that holds no number is zero.

## 예제

```ahk
FileAppend A_KeyDelay, "*"
```
