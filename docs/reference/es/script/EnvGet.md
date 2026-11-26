# `EnvGet`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · El analizador y la ejecución del script

## Descripción

Answers the value of an environment variable of the runner; the environment is the one of the process the script runs in.

## Sintaxis

```ahk
V := EnvGet(Name)
```

## Parámetros

- **Name** — The variable.

## Valor de retorno

the value of the variable, or the empty text when the environment does not have it

## Errores

nothing; a name that is not there is the empty text and not a failure.

## Ejemplo

```ahk
home := EnvGet("HOME")
```

## Relacionados

[`EnvSet`](EnvSet.md)
