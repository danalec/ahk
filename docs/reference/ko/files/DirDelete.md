# `DirDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 파일과 디렉터리 작업

## 설명

Takes a directory away. A directory that is not empty stays without it unless the recursion is asked for.

## 구문

```ahk
DirDelete Path, Recurse
```

## 매개변수

- **Path** — The directory.
- **Recurse** — 1 to delete contents.

## 반환 값

nothing; the name of a command answers nothing.

## 오류

Error `Failed` when the directory is not there, or when it is not empty and the recursion is not asked for.

## 예제

```ahk
d := DirDelete("out", 1)
```

## 관련

[`DirCreate`](DirCreate.md), [`FileDelete`](FileDelete.md)
