# `FileGetSize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파일과 디렉터리 작업

## 설명

Answers how big a file is.

## 구문

```ahk
FileGetSize …
```

## 매개변수

FileName, [Unit]

## 반환 값

the size, rounded down; a folder answers zero.

## 오류

ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

## 예제

```ahk
n := FileGetSize("out.txt", "K")
```
