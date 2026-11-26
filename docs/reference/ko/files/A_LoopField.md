# `A_LoopField`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · variable · 파일과 디렉터리 작업

## 설명

The field the parse loop is on: the piece of the text between two delimiters.

## 구문

```ahk
A_LoopField 
```

## 매개변수

nothing; it is a variable, not a call

## 반환 값

the field, as a text; the empty text before the first parse loop

## 오류

nothing to read; the write is refused.

## 예제

```ahk
Loop Parse "a,b,c", ","
    FileAppend A_LoopField, "*"
```
