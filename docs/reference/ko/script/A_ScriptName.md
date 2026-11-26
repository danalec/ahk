# `A_ScriptName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · variable · 파서와 스크립트 실행

## 설명

The name of the script. The script of the runner comes on the standard input and has no file, so the name is an asterisk. A script may write the name, and the value written is the one it keeps.

## 구문

```ahk
A_ScriptName 
```

## 매개변수

nothing; it is a variable, not a call

## 반환 값

the text "*", or the text that was written

## 오류

nothing to read; the write takes any value.

## 예제

```ahk
FileAppend A_ScriptName, "*"
```
