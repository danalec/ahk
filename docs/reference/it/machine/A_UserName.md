# `A_UserName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · variable · La macchina, i processi e l'ambiente

## Descrizione

The name of the user of the session: the `USER` of the environment.

## Sintassi

```ahk
A_UserName 
```

## Parametri

nothing; it is a variable, not a call

## Valore di ritorno

the name, as a text

## Errori

the read fails when the session carries no such name: "the session does not carry the name of the user: the environment has no USER."; the write is refused.

## Esempio

```ahk
FileAppend A_UserName, "*"
```
