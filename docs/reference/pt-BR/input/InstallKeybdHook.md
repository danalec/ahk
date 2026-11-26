# `InstallKeybdHook`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings e entrada

## Descrição

Responde que o hook está lá. Os hooks de v2 são instalados quando um hotkey precisa deles, e o backend vigia o teclado desde o primeiro hotkey, então o comando diz que o hook já existe.

## Sintaxe

```ahk
InstallKeybdHook …
```

## Parâmetros

[OnOff] [HookName]

## Valor de retorno

nada

## Erros

none; the command never fails.

## Exemplo

```ahk
InstallKeybdHook
```
