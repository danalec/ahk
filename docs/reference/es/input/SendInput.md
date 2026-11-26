# `SendInput`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings y entrada

## Descripción

Sends the keys the way the box sends them: the input method of v2 is the method of the keyboard of the box, so the keys go through it, the way `Send` sends them.

## Sintaxis

```ahk
SendInput …
```

## Parámetros

Keys

## Valor de retorno

nothing

## Errores

the keyboard of the backend is a Linux uinput device, and it fails on a host that is not Linux.

## Ejemplo

```ahk
SendInput "{Enter}"
```
