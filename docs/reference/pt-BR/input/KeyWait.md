# `KeyWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings e entrada

## Descrição

Recusado. A árvore não guarda as teclas que estão pressionadas, então não pode esperar por uma.

## Sintaxe

```ahk
KeyWait KeyName, Options
```

## Parâmetros

- **KeyName** — The key.
- **Options** — D (down), T timeout, L logical.

## Valor de retorno

nada; a instrução falha

## Erros

this runtime does not keep the keys that are down, so it cannot wait for one.

## Exemplo

```ahk
KeyWait "a"
```

## Relacionados

[`GetKeyState`](GetKeyState.md), [`Hotkey`](Hotkey.md)
