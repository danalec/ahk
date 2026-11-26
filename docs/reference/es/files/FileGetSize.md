# `FileGetSize`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Operaciones de archivos y directorios

## Descripción

Answers how big a file is.

## Sintaxis

```ahk
FileGetSize …
```

## Parámetros

FileName, [Unit]

## Valor de retorno

the size, rounded down; a folder answers zero.

## Errores

ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

## Ejemplo

```ahk
n := FileGetSize("out.txt", "K")
```
