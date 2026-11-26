# `ProcessSetPriority`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · La máquina, los procesos y el entorno

## Descripción

Sets the priority of a process.

## Sintaxis

```ahk
ProcessSetPriority Priority, NameOrPID
```

## Parámetros

- **Priority** — L/B/N/A/H/R or a number.
- **NameOrPID** — The target.

## Valor de retorno

Nothing.

## Ejemplo

```ahk
ProcessSetPriority "H", pid
```

## Relacionados

[`Process`](Process.md)
