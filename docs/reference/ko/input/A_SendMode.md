# `A_SendMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · variable · 핫키, 핫스트링, 입력

## 설명

The method that a `Send` uses: `Input`, `Event`, `Play` or `InputThenPlay`.

## 구문

```ahk
A_SendMode 
```

## 매개변수

nothing; it is a variable, not a call

## 반환 값

the name of the mode, as a text

## 오류

the write takes the name of one of the four modes, and any other name is refused with "Invalid value.".

## 예제

```ahk
FileAppend A_SendMode, "*"
```
