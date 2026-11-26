# `A_ScriptDir`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · variable · Ayrıştırıcı ve betiğin çalıştırılması

## Açıklama

The directory where the run started. The script of the runner arrives on the standard input and has no file of its own, so the directory of the script is the directory of the run.

## Sözdizimi

```ahk
A_ScriptDir 
```

## Parametreler

nothing; it is a variable, not a call

## Dönüş değeri

the path of the directory, as a text

## Hatalar

the read fails when the runtime does not keep the directory where the run started; the write is refused.

## Örnek

```ahk
FileAppend A_ScriptDir, "*"
```
