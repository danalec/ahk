# `A_ComputerName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · variable · 機器、行程與環境

## 描述

The name of the computer, as the network sees it: the kernel, and the `HOSTNAME` of the environment when the kernel has no name.

## 語法

```ahk
A_ComputerName 
```

## 參數

nothing; it is a variable, not a call

## 傳回值

the name, as a text

## 錯誤

the read fails when neither the kernel nor the session carries the name; the write is refused.

## 範例

```ahk
FileAppend A_ComputerName, "*"
```
