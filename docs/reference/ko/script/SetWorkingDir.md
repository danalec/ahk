# `SetWorkingDir`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파서와 스크립트 실행

## 설명

Sets the working directory of the script. A name that is not absolute is a folder under the working directory, the way the dialect says it.

## 구문

```ahk
SetWorkingDir …
```

## 매개변수

Directory

## 반환 값

the integer 1

## 오류

an empty name is refused with "the name of the directory is missing.", and a folder that cannot be entered fails with the reason of the desktop.

## 예제

```ahk
SetWorkingDir "."
```
