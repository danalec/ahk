# `EnvSet`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · El analizador y la ejecución del script

## Descripción

Sets an environment variable of the runner and answers the value; the script runs on one thread, so nothing else reads the environment while the change happens.

## Sintaxis

```ahk
EnvSet Name, Value
```

## Parámetros

- **Name** — The variable.
- **Value** — The value; omitted deletes.

## Valor de retorno

the value that was set, as text

## Errores

an empty name and a name that carries an equal sign are refused with the reason of the machine.

## Ejemplo

```ahk
EnvSet "MY_VAR", "value"
```

## Relacionados

[`EnvGet`](EnvGet.md)
