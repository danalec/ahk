# `ProcessWaitClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · A máquina, os processos e o ambiente

## Descrição

Espera que um processo que existe termine; um processo que nunca existiu já sumiu, e a espera acaba na hora.

## Sintaxe

```ahk
ProcessWaitClose …
```

## Parâmetros

ProcessNameOrPID, [Timeout]

## Valor de retorno

0 quando o processo sumiu, o pid do processo que ainda está lá no fim do tempo

## Erros

a call without a timeout waits for ever; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## Exemplo

```ahk
ProcessWaitClose "firefox", 10
```
