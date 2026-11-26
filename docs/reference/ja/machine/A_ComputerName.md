# `A_ComputerName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · variable · マシン、プロセス、環境

## 説明

The name of the computer, as the network sees it: the kernel, and the `HOSTNAME` of the environment when the kernel has no name.

## 構文

```ahk
A_ComputerName 
```

## パラメータ

nothing; it is a variable, not a call

## 戻り値

the name, as a text

## エラー

the read fails when neither the kernel nor the session carries the name; the write is refused.

## 例

```ahk
FileAppend A_ComputerName, "*"
```
