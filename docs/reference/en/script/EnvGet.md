# `EnvGet`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · Parser and script execution

## Description

Answers the value of an environment variable of the runner; the environment is the one of the process the script runs in.

## Syntax

```ahk
V := EnvGet(Name)
```

## Parameters

- **Name** — The variable.

## Return value

the value of the variable, or the empty text when the environment does not have it

## Errors

nothing; a name that is not there is the empty text and not a failure.

## Example

```ahk
home := EnvGet("HOME")
```

## Related

[`EnvSet`](EnvSet.md)
