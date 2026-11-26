# `A_ScriptName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · variable · Ayrıştırıcı ve betiğin çalıştırılması

## Açıklama

The name of the script. The script of the runner comes on the standard input and has no file, so the name is an asterisk. A script may write the name, and the value written is the one it keeps.

## Sözdizimi

```ahk
A_ScriptName 
```

## Parametreler

nothing; it is a variable, not a call

## Dönüş değeri

the text "*", or the text that was written

## Hatalar

nothing to read; the write takes any value.

## Örnek

```ahk
FileAppend A_ScriptName, "*"
```
