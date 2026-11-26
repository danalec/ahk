# `ProcessGetParent`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · A máquina, os processos e o ambiente

## Descrição

The parent PID of a process.

## Sintaxe

```ahk
N := ProcessGetParent(NameOrPID)
```

## Parâmetros

- **NameOrPID** — The target.

## Valor de retorno

The parent PID.

## Exemplo

```ahk
pp := ProcessGetParent(pid)
```

## Relacionados

[`ProcessExist`](ProcessExist.md)
