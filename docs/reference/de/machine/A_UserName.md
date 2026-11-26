# `A_UserName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · variable · Die Maschine, die Prozesse und die Umgebung

## Beschreibung

The name of the user of the session: the `USER` of the environment.

## Syntax

```ahk
A_UserName 
```

## Parameter

nothing; it is a variable, not a call

## Rückgabewert

the name, as a text

## Fehler

the read fails when the session carries no such name: "the session does not carry the name of the user: the environment has no USER."; the write is refused.

## Beispiel

```ahk
FileAppend A_UserName, "*"
```
