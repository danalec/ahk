# `MouseClick`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings e entrada

## Descrição

Clica com o ponteiro do backend. O botão é o primeiro parâmetro, como na função de v2, e um botão ausente é o esquerdo.

## Sintaxe

```ahk
MouseClick …
```

## Parâmetros

Button, X, Y [Count] [Options]

## Valor de retorno

nada

## Erros

the wheel, the fourth and the fifth buttons are refused as not carried yet.

## Exemplo

```ahk
MouseClick "Right", 100, 200
```
