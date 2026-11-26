# `DirExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 파일과 디렉터리 작업

## 설명

Answers the attributes of the first folder that the name or the pattern matches.

## 구문

```ahk
A := DirExist(Path)
```

## 매개변수

- **Path** — A path or wildcard.

## 반환 값

`D` when the folder is there, and the empty text when it is not; a file that matches the pattern is not a folder.

## 오류

nothing fails.

## 예제

```ahk
x := DirExist("out")
```

## 관련

[`FileExist`](FileExist.md)
