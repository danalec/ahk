# `FileDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파일과 디렉터리 작업

## 설명

Takes a file away, or every file that a pattern names. A pattern that matches nothing is not a failure.

## 구문

```ahk
FileDelete Pattern
```

## 매개변수

- **Pattern** — A path or wildcard.

## 반환 값

nothing

## 오류

OSError when the file cannot be taken away.

## 예제

```ahk
FileDelete "out.txt"
```

## 관련

[`DirDelete`](DirDelete.md), [`FileRecycle`](FileRecycle.md)
