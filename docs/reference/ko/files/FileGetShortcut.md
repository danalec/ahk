# `FileGetShortcut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파일과 디렉터리 작업

## 설명

Reads a shortcut's fields.

## 구문

```ahk
FileGetShortcut Path, OutTarget, OutDir, OutArgs, OutDesc, OutIcon, OutIconNum, OutRunState
```

## 매개변수

1..255.

## 반환 값

Nothing; the vars are filled.

## 예제

```ahk
FileGetShortcut "a.lnk", &t
```

## 관련

[`FileCreateShortcut`](FileCreateShortcut.md)
