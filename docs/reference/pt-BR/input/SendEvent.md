# `SendEvent`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings e entrada

## Descrição

Envia as teclas do jeito do backend. O método de evento de v2 é o método do Windows, e a árvore tem só um teclado, então as teclas passam por ele como `Send` faz.

## Sintaxe

```ahk
SendEvent …
```

## Parâmetros

Keys

## Valor de retorno

nada

## Erros

the keyboard of the backend is a Linux uinput device, and it fails on a host that is not Linux.

## Exemplo

```ahk
SendEvent "^a"
```
