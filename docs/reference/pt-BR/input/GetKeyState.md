# `GetKeyState`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Hotkeys, hotstrings e entrada

## Descrição

Lê o que o vigia de hotkeys viu: se uma tecla está pressionada. O segundo parâmetro só aceita 'T' e 'P' de v2.

## Sintaxe

```ahk
S := GetKeyState(KeyName, Mode)
```

## Parâmetros

- **KeyName** — The key.
- **Mode** — P physical, T toggle.

## Valor de retorno

1 quando a tecla está pressionada, 0 quando não está

## Erros

a key name the tree does not know, and a mode that is neither 'T' nor 'P', are refused as invalid parameters; on a host that is not Linux the watcher cannot answer at all.

## Exemplo

```ahk
state := GetKeyState("Shift")
```

## Relacionados

[`KeyWait`](KeyWait.md), [`SetCapsLockState`](SetCapsLockState.md)
