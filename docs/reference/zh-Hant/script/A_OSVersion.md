# `A_OSVersion`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · variable · 解析器與腳本執行

## 描述

The version of the operating system of the machine: the release of the kernel, read from `/proc/sys/kernel/osrelease`.

## 語法

```ahk
A_OSVersion 
```

## 參數

nothing; it is a variable, not a call

## 傳回值

the version, as a text

## 錯誤

the read fails when the release of the kernel cannot be read; the write is refused.

## 範例

```ahk
FileAppend A_OSVersion, "*"
```
