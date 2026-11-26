# `File`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · method · `Error` · 파일과 디렉터리 작업

## 설명

The class of an open file of the machine. The class is not made: `File(...)` answers "Invalid base."; the file is made by the `FileOpen` built-in, and it is the only way to have one.

## 구문

```ahk
File 
```

## 매개변수

nothing that makes it; the arguments of `FileOpen` open the file

## 반환 값

the object of the class `File`

## 오류

the call of the class is refused with "Invalid base.".

## 예제

```ahk
f := FileOpen("notes.txt", "w")
f.Close()
```
