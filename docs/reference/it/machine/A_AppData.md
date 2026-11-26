# `A_AppData`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · variable · La macchina, i processi e l'ambiente

## Descrizione

The folder of the data of the user. On a Linux box it is `$XDG_DATA_HOME`, and `$HOME/.local/share` when the variable is not there.

## Sintassi

```ahk
A_AppData 
```

## Parametri

nothing; it is a variable, not a call

## Valore di ritorno

the path of the folder, as a text

## Errori

the read fails with "the machine has no such folder." when the machine has no such folder; the write is refused.

## Esempio

```ahk
FileAppend A_AppData, "*"
```
