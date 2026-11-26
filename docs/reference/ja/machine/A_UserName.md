# `A_UserName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · variable · マシン、プロセス、環境

## 説明

The name of the user of the session: the `USER` of the environment.

## 構文

```ahk
A_UserName 
```

## パラメータ

nothing; it is a variable, not a call

## 戻り値

the name, as a text

## エラー

the read fails when the session carries no such name: "the session does not carry the name of the user: the environment has no USER."; the write is refused.

## 例

```ahk
FileAppend A_UserName, "*"
```
