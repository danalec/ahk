# `A_OSVersion`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · variable · Der Parser und die Ausführung des Skripts

## Beschreibung

The version of the operating system of the machine: the release of the kernel, read from `/proc/sys/kernel/osrelease`.

## Syntax

```ahk
A_OSVersion 
```

## Parameter

nothing; it is a variable, not a call

## Rückgabewert

the version, as a text

## Fehler

the read fails when the release of the kernel cannot be read; the write is refused.

## Beispiel

```ahk
FileAppend A_OSVersion, "*"
```
