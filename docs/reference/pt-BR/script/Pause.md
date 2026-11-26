# `Pause`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · O analisador e a execução do script

## Descrição

Define o estado de pausa do script. A árvore guarda um estado em vez de esperar uma tecla, o que travaria um script sem teclado. O estado aparece em A_IsPaused.

## Sintaxe

```ahk
Pause …
```

## Parâmetros

[OnOff]

## Valor de retorno

texto vazio

## Erros

a parameter that is not 0, 1 or -1 is an invalid parameter.

## Exemplo

```ahk
Pause true
```
