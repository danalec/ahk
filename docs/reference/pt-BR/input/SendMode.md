# `SendMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings e entrada

## Descrição

Diz como as teclas são enviadas. As teclas desta árvore passam uma a uma pelo teclado do backend, e esse é o modo do backend; os outros modos de v2 são os do Windows.

## Sintaxe

```ahk
SendMode …
```

## Parâmetros

Method

## Valor de retorno

o número 1 para o modo do backend

## Erros

a mode that is not 'Input' fails with 'the keys of this runtime go through the keyboard of the box, and that is the only mode there is.'

## Exemplo

```ahk
SendMode "Input"
```
