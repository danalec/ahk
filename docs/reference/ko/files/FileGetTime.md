# `FileGetTime`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파일과 디렉터리 작업

## 설명

Answers when a file was made, when it was changed, or when it was read.

## 구문

```ahk
FileGetTime …
```

## 매개변수

FileName, [Which]

## 반환 값

the stamp of v2, the fourteen digits of `YYYYMMDDHH24MISS`, in the local time of the machine.

## 오류

ValueError when `Which` is not `M`, `C`, `A` or `CUT`; OSError when the file cannot be read.

## 예제

```ahk
t := FileGetTime("out.txt", "M")
```
