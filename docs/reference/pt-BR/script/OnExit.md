# `OnExit`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · O analisador e a execução do script

## Descrição

Registra a função que roda quando o script termina, qualquer que seja o motivo: a última instrução, um código pedido pelo script, ou uma falha que o parou.

## Sintaxe

```ahk
OnExit Function, Add
```

## Parâmetros

- **Function** — The callback (ExitReason, ExitCode).
- **Add** — 1 to add, -1 to remove.

## Valor de retorno

o inteiro 1

## Erros

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## Exemplo

```ahk
OnExit((reason, code) => 0)
```

## Relacionados

[`ExitApp`](ExitApp.md), [`OnError`](OnError.md)
