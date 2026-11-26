# `ProcessClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · A máquina, os processos e o ambiente

## Descrição

Encerra um processo; a caixa envia SIGTERM, o sinal educado que pede ao processo que termine.

## Sintaxe

```ahk
ProcessClose NameOrPID
```

## Parâmetros

- **NameOrPID** — The name or PID.

## Valor de retorno

o pid do processo encerrado, 0 quando ele não estava lá ou não pôde ser encerrado

## Erros

a process of another user is a process the script cannot open, and the answer is zero and not a failure.

## Exemplo

```ahk
ProcessClose "firefox"
```

## Relacionados

[`ProcessExist`](ProcessExist.md), [`WinKill`](WinKill.md)
