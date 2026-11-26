# `FileSelect`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · Operaciones de archivos y directorios

## Descripción

Shows a file-selection dialog.

## Sintaxis

```ahk
S := FileSelect(Options, RootDir, Prompt, Filter)
```

## Parámetros

0..4.

## Valor de retorno

The path(s) or empty.

## Ejemplo

```ahk
p := FileSelect()
```

## Relacionados

[`DirSelect`](DirSelect.md), [`InputBox`](InputBox.md)
