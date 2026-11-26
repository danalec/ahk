# `RunScript`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*KeySharp* · function · El analizador y la ejecución del script

## Descripción

Starts a script in a new process and answers a ScriptProcess (ks).

## Sintaxis

```ahk
P := RunScript(Path, Args)
```

## Parámetros

- **Path** — The script.
- **Args** — The arguments.

## Valor de retorno

A ScriptProcess.

## Ejemplo

```ahk
p := RunScript("child.ahk")
```

## Relacionados

[`ScriptProcess`](ScriptProcess.md), [`Run`](Run.md)
