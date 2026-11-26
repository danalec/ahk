# `FileSetTime`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파일과 디렉터리 작업

## 설명

Changes the time of a file. The time comes first and the name of the file second, the way the binary of v2.0.31 takes them; a time that is blank is the moment of the machine. The name of the file may be a pattern, and the time is changed in every file that the pattern names.

## 구문

```ahk
FileSetTime …
```

## 매개변수

TimeStamp, FileName, [Which], [Recurse]

## 반환 값

nothing

## 오류

ValueError when the stamp is not one of v2, or when the time that is wanted is not the "M", the "C" or the "A"; OSError when the times of the file cannot be changed. The fourth parameter is the recursion: the "R" walks into the subfolders, and any other letter is refused.

## 예제

```ahk
FileSetTime "20000101000000", "out.txt", "M"
```
