# `RunWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · A máquina, os processos e o ambiente

## Descrição

Executa um comando e espera por ele, do mesmo jeito que Run. O nome é a forma de função do mesmo tratador.

## Sintaxe

```ahk
ExitCode := RunWait(Target, WorkingDir, Options, OutputVarPID)
```

## Parâmetros

- **Target** — The program or document.
- **WorkingDir** — The child's directory.
- **Options** — Min/Max/Hide.
- **OutputVarPID** — Receives the PID.

## Valor de retorno

o código de saída do comando

## Erros

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## Observações

Blocks the script until the child exits; timers do not run during the wait on this tree.

## Exemplo

```ahk
RunWait "ls"
```

## Relacionados

[`Run`](Run.md), [`Process`](Process.md)
