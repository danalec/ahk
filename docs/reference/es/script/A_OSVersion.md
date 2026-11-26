# `A_OSVersion`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · variable · El analizador y la ejecución del script

## Descripción

The version of the operating system of the machine: the release of the kernel, read from `/proc/sys/kernel/osrelease`.

## Sintaxis

```ahk
A_OSVersion 
```

## Parámetros

nothing; it is a variable, not a call

## Valor de retorno

the version, as a text

## Errores

the read fails when the release of the kernel cannot be read; the write is refused.

## Ejemplo

```ahk
FileAppend A_OSVersion, "*"
```
