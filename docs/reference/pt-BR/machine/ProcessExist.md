# `ProcessExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · A máquina, os processos e o ambiente

## Descrição

Responde se existe um processo e com qual pid; os processos da máquina são lidos de /proc.

## Sintaxe

```ahk
PID := ProcessExist(NameOrPID)
```

## Parâmetros

- **NameOrPID** — The name or a PID.

## Valor de retorno

o pid do processo, ou 0 quando não há nenhum

## Erros

a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none"; a process that is not there is zero and not a failure.

## Exemplo

```ahk
if ProcessExist("firefox")
```

## Relacionados

[`ProcessClose`](ProcessClose.md), [`ProcessWait`](ProcessWait.md), [`Run`](Run.md)
