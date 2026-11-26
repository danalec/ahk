# `A_OSVersion`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · variable · L'analyseur et l'exécution du script

## Description

The version of the operating system of the machine: the release of the kernel, read from `/proc/sys/kernel/osrelease`.

## Syntaxe

```ahk
A_OSVersion 
```

## Paramètres

nothing; it is a variable, not a call

## Valeur de retour

the version, as a text

## Erreurs

the read fails when the release of the kernel cannot be read; the write is refused.

## Exemple

```ahk
FileAppend A_OSVersion, "*"
```
