# `FileGetShortcut`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Operaciones de archivos y directorios

## Descripción

Reads a shortcut's fields.

## Sintaxis

```ahk
FileGetShortcut Path, OutTarget, OutDir, OutArgs, OutDesc, OutIcon, OutIconNum, OutRunState
```

## Parámetros

1..255.

## Valor de retorno

Nothing; the vars are filled.

## Ejemplo

```ahk
FileGetShortcut "a.lnk", &t
```

## Relacionados

[`FileCreateShortcut`](FileCreateShortcut.md)
