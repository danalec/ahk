# `Thread`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · O analisador e a execução do script

## Descrição

O estado da thread da execução. O comando é "Priority", "Interrupt" ou "NoTimers", e a palavra é lida sem olhar para as maiúsculas: "Priority" toma a prioridade da thread, "Interrupt" toma os milissegundos e as linhas da janela da interrupção, e "NoTimers" diz se os timers podem rodar -- a negação dupla do v2, em que "NoTimers" é zero quando os timers podem rodar.

## Sintaxe

```ahk
Thread …
```

## Parâmetros

Command, [Value1], [Value2]

## Valor de retorno

texto vazio

## Erros

a command that is not one of the three refuses with "Parameter #1 of Thread is invalid.", and the word is in the `Extra`; a value that is not there leaves the setting as it was.

## Exemplo

```ahk
Thread "Priority", 5
```
