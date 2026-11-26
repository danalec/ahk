# `FileGetTime`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Operazioni su file e directory

## Descrizione

Answers when a file was made, when it was changed, or when it was read.

## Sintassi

```ahk
FileGetTime …
```

## Parametri

FileName, [Which]

## Valore di ritorno

the stamp of v2, the fourteen digits of `YYYYMMDDHH24MISS`, in the local time of the machine.

## Errori

ValueError when `Which` is not `M`, `C`, `A` or `CUT`; OSError when the file cannot be read.

## Esempio

```ahk
t := FileGetTime("out.txt", "M")
```
