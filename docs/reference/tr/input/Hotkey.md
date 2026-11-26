# `Hotkey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Kısayollar, tetikleme dizeleri ve giriş

## Açıklama

Registers a hotkey: the key it watches for and the function it runs. The runtime owns the registry and the machine owns the watcher: the script says which function a hotkey runs, and the machine says which key was pressed.

## Sözdizimi

```ahk
Hotkey KeyName, Action, Options
```

## Parametreler

- **KeyName** — e.g. ^!a, ~LButton.
- **Action** — A function/label, or On/Off/Toggle/AltTab etc.
- **Options** — On/Off/UseErrorLevel/MaxThreadsPerHotkey.

## Dönüş değeri

nothing

## Hatalar

a hotkey without an action is 'Nonexistent hotkey.'; the watcher reads the physical keyboard of the machine, so on a host that is not Linux the registration fails with 'the hotkeys of this box watch a Linux device; this is not Linux'.

## Notlar

On Linux the keyboard is watched (/dev/input or X11); on Windows RegisterHotKey or the hook.

## Örnek

```ahk
Hotkey "^s", SaveTheFile
```

## İlgili

[`Hotstring`](Hotstring.md), [`#HotkeyInterval`](#HotkeyInterval.md), [`KeyWait`](KeyWait.md)
