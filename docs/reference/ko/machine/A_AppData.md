# `A_AppData`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · variable · 머신, 프로세스, 환경

## 설명

The folder of the data of the user. On a Linux box it is `$XDG_DATA_HOME`, and `$HOME/.local/share` when the variable is not there.

## 구문

```ahk
A_AppData 
```

## 매개변수

nothing; it is a variable, not a call

## 반환 값

the path of the folder, as a text

## 오류

the read fails with "the machine has no such folder." when the machine has no such folder; the write is refused.

## 예제

```ahk
FileAppend A_AppData, "*"
```
