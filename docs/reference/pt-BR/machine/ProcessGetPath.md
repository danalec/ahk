# `ProcessGetPath`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · A máquina, os processos e o ambiente

## Descrição

Devolve o caminho inteiro do executável de um processo; o caminho é o alvo da ligação /proc/<pid>/exe, e o " (deleted)" que o kernel acrescenta é cortado.

## Sintaxe

```ahk
ProcessGetPath …
```

## Parâmetros

[ProcessNameOrPID]

## Valor de retorno

o caminho do executável, como texto

## Erros

"Target process not found." when the process is not there; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## Exemplo

```ahk
path := ProcessGetPath(1234)
```
