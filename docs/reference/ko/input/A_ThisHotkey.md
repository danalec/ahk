# `A_ThisHotkey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · variable · 핫키, 핫스트링, 입력

## 설명

The name of the hotkey that runs. The hotkeys are the ones of the hotkey system of the runtime, and the name is the empty text when no hotkey runs.

## 구문

```ahk
A_ThisHotkey 
```

## 매개변수

nothing; it is a variable, not a call

## 반환 값

the name of the hotkey, as a text; the empty text when no hotkey runs

## 오류

nothing to read; the write is refused.

## 예제

```ahk
FileAppend A_ThisHotkey, "*"
```
