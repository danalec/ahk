# `SendPlay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings e entrada

## Descrição

Recusado. O método play de v2 não envia teclas: ele posta as mensagens na janela ativa, e aqui não há janela para receber.

## Sintaxe

```ahk
SendPlay …
```

## Parâmetros

Keys

## Valor de retorno

nada; a instrução falha

## Erros

the play method of v2 posts the messages to the active window, and there is no window here to post them to.

## Exemplo

```ahk
SendPlay "{Enter}"
```
