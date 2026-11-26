# `Reload`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파서와 스크립트 실행

## 설명

Starts the script again in a process of its own: the old instance ends and the new one carries on with the same file and the same arguments. A script that comes from the standard input has no file, and the v2 does nothing then.

## 구문

```ahk
Reload
```

## 매개변수

none

## 반환 값

the empty text

## 오류

none; the name takes no parameter, and a script without a file is not refused -- the call simply does nothing.

## 설명

The new instance replaces the old; single-instance rules apply.

## 예제

```ahk
Reload
```

## 관련

[`ExitApp`](ExitApp.md), [`#SingleInstance`](#SingleInstance.md)
