# `FileGetVersion`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파일과 디렉터리 작업

## 설명

The version of a file.

## 구문

```ahk
S := FileGetVersion(Filename)
```

## 매개변수

1..255.

## 반환 값

The version.

## 예제

```ahk
v := FileGetVersion("a.exe")
```

## 관련

[`FileExist`](FileExist.md)
