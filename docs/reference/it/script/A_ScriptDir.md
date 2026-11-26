# `A_ScriptDir`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · variable · L'analizzatore e l'esecuzione dello script

## Descrizione

The directory where the run started. The script of the runner arrives on the standard input and has no file of its own, so the directory of the script is the directory of the run.

## Sintassi

```ahk
A_ScriptDir 
```

## Parametri

nothing; it is a variable, not a call

## Valore di ritorno

the path of the directory, as a text

## Errori

the read fails when the runtime does not keep the directory where the run started; the write is refused.

## Esempio

```ahk
FileAppend A_ScriptDir, "*"
```
