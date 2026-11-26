# `A_ComputerName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · variable · La macchina, i processi e l'ambiente

## Descrizione

The name of the computer, as the network sees it: the kernel, and the `HOSTNAME` of the environment when the kernel has no name.

## Sintassi

```ahk
A_ComputerName 
```

## Parametri

nothing; it is a variable, not a call

## Valore di ritorno

the name, as a text

## Errori

the read fails when neither the kernel nor the session carries the name; the write is refused.

## Esempio

```ahk
FileAppend A_ComputerName, "*"
```
