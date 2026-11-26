# `Reload`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · O analisador e a execução do script

## Descrição

Inicia o script de novo em um processo só dele: a instância antiga termina e a nova continua com o mesmo arquivo e os mesmos argumentos. Um script que vem da entrada padrão não tem arquivo, e o v2 não faz nada nesse caso.

## Sintaxe

```ahk
Reload
```

## Parâmetros

none

## Valor de retorno

texto vazio

## Erros

none; the name takes no parameter, and a script without a file is not refused -- the call simply does nothing.

## Observações

The new instance replaces the old; single-instance rules apply.

## Exemplo

```ahk
Reload
```

## Relacionados

[`ExitApp`](ExitApp.md), [`#SingleInstance`](#SingleInstance.md)
