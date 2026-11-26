# `FileSelect`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 파일과 디렉터리 작업

## 설명

Shows a file-selection dialog.

## 구문

```ahk
S := FileSelect(Options, RootDir, Prompt, Filter)
```

## 매개변수

0..4.

## 반환 값

The path(s) or empty.

## 예제

```ahk
p := FileSelect()
```

## 관련

[`DirSelect`](DirSelect.md), [`InputBox`](InputBox.md)
