# `Send`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings e entrada

## Descrição

Digita texto e pressiona teclas na máquina, pelo teclado uinput do backend.

## Sintaxe

```ahk
Send Keys
```

## Parâmetros

- **Keys** — The keystrokes, with {Enter}, ^c, !tab, {Blind}, etc.

## Valor de retorno

nada

## Erros

the keyboard is a Linux uinput device, so on a host that is not Linux every key action fails; a character the US layout has no key for fails with 'the keyboard of this box has no key for'.

## Observações

SendMode chooses Input/Play/Event. On Linux it is the uinput/XTEST keyboard.

## Exemplo

```ahk
Send "hello{Enter}"
```

## Relacionados

[`SendText`](SendText.md), [`ControlSend`](ControlSend.md), [`Click`](Click.md), [`SendMode`](SendMode.md)
