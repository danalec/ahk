# `EnvSet`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · Parser and script execution

## Description

Sets an environment variable of the runner and answers the value; the script runs on one thread, so nothing else reads the environment while the change happens.

## Syntax

```ahk
EnvSet Name, Value
```

## Parameters

- **Name** — The variable.
- **Value** — The value; omitted deletes.

## Return value

the value that was set, as text

## Errors

an empty name and a name that carries an equal sign are refused with the reason of the machine.

## Example

```ahk
EnvSet "MY_VAR", "value"
```

## Related

[`EnvGet`](EnvGet.md)
