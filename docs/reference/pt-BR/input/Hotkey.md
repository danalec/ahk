# `Hotkey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings e entrada

## Descrição

Registra um hotkey: a tecla vigiada e a função que ela roda. O registro é da árvore e o vigia é da máquina: o script diz qual função o hotkey roda, e a máquina diz qual tecla foi pressionada.

## Sintaxe

```ahk
Hotkey KeyName, Action, Options
```

## Parâmetros

- **KeyName** — e.g. ^!a, ~LButton.
- **Action** — A function/label, or On/Off/Toggle/AltTab etc.
- **Options** — On/Off/UseErrorLevel/MaxThreadsPerHotkey.

## Valor de retorno

nada

## Erros

a hotkey without an action is 'Nonexistent hotkey.'; the watcher reads the physical keyboard of the machine, so on a host that is not Linux the registration fails with 'the hotkeys of this box watch a Linux device; this is not Linux'.

## Observações

On Linux the keyboard is watched (/dev/input or X11); on Windows RegisterHotKey or the hook.

## Exemplo

```ahk
Hotkey "^s", SaveTheFile
```

## Relacionados

[`Hotstring`](Hotstring.md), [`#HotkeyInterval`](#HotkeyInterval.md), [`KeyWait`](KeyWait.md)
