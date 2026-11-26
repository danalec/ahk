# `Sleep`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파서와 스크립트 실행

## 설명

Pauses the script for a number of milliseconds.

## 구문

```ahk
Sleep Milliseconds
```

## 매개변수

- **Milliseconds** — 0..60000; a larger value is clamped to 60000, a negative one sleeps 0.

## 반환 값

the number of milliseconds waited

## 오류

none; a wait longer than 60000 milliseconds is cut at that number.

## 설명

The pause is clamped, not refused. During the sleep the timers of the script may still run (the runtime checks them between statements).

## 예제

```ahk
Sleep 500
```

## 관련

[`SetTimer`](SetTimer.md), [`Critical`](Critical.md)
