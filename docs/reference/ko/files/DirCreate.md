# `DirCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 파일과 디렉터리 작업

## 설명

Makes a directory, with the folders that are missing in the way.

## 구문

```ahk
DirCreate Path
```

## 매개변수

- **Path** — The directory.

## 반환 값

nothing

## 오류

OSError when the directory cannot be made.

## 예제

```ahk
DirCreate "out"
```

## 관련

[`DirDelete`](DirDelete.md), [`DirCopy`](DirCopy.md)
