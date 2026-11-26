# `ExitApp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · O analisador e a execução do script

## Descrição

Encerra o script com um código. O código vai para o exit_code do relatório.

## Sintaxe

```ahk
ExitApp ExitCode
```

## Parâmetros

- **ExitCode** — The code the process exits with (default 0).

## Valor de retorno

o código

## Erros

none.

## Observações

OnExit callbacks run before the process ends.

## Exemplo

```ahk
ExitApp 3
```

## Relacionados

[`Exit`](Exit.md), [`Reload`](Reload.md), [`OnExit`](OnExit.md)
