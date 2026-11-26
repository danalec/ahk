# `FileMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파일과 디렉터리 작업

## 설명

Moves a file, or every file that a pattern names. The source and the destination are the ones of `FileCopy`.

## 구문

```ahk
FileMove Source, Dest, Overwrite
```

## 매개변수

- **Source** — Path/wildcard.
- **Dest** — Destination.
- **Overwrite** — 1 to overwrite.

## 반환 값

nothing

## 오류

Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

## 예제

```ahk
FileMove "a.txt", "b.txt"
```

## 관련

[`FileCopy`](FileCopy.md), [`DirMove`](DirMove.md)
