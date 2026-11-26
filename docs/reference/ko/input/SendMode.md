# `SendMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 핫키, 핫스트링, 입력

## 설명

Says how the keys are sent. The keys of this runtime go through the keyboard of the box, one at a time, and that is the mode of the box; the other modes of v2 are the ones of Windows.

## 구문

```ahk
SendMode …
```

## 매개변수

Method

## 반환 값

the number 1 for the mode of the box

## 오류

a mode that is not 'Input' fails with 'the keys of this runtime go through the keyboard of the box, and that is the only mode there is.'

## 예제

```ahk
SendMode "Input"
```
