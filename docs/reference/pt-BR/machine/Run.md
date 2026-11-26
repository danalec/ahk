# `Run`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · A máquina, os processos e o ambiente

## Descrição

Executa um comando e espera por ele. O que o comando escreveu na saída vai para a saída do relatório.

## Sintaxe

```ahk
Run Target, WorkingDir, Options, OutputVarPID
```

## Parâmetros

- **Target** — A program, document, URL or verb.
- **WorkingDir** — The working directory of the child.
- **Options** — Min/Max/Hide/UseErrorLevel.
- **OutputVarPID** — A variable that receives the process id.

## Valor de retorno

o código de saída do comando

## Erros

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## Observações

On Linux it is sh -c; on Windows ShellExecute/CreateProcess. The child outlives the script unless killed.

## Exemplo

```ahk
Run "ls"
```

## Relacionados

[`RunWait`](RunWait.md), [`Process`](Process.md), [`WinWait`](WinWait.md)
