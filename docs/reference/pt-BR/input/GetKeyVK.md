# `GetKeyVK`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings e entrada

## Descrição

O código de tecla virtual do Windows da tecla. É o número da tecla no teclado do Windows, e a árvore o mantém para os scripts que o pedem.

## Sintaxe

```ahk
GetKeyVK …
```

## Parâmetros

KeyName

## Valor de retorno

o código de tecla virtual, um número

## Erros

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## Exemplo

```ahk
vk := GetKeyVK("Enter")
```
