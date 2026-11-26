# `Shutdown`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · マシン、プロセス、環境

## 説明

Shuts down / logs off / reboots the machine.

## 構文

```ahk
Shutdown Code
```

## パラメータ

- **Code** — The shutdown flags (1 logoff, 2 reboot, 8 power-down...).

## 戻り値

Nothing.

## 例

```ahk
Shutdown 2
```

## 関連

[`ExitApp`](ExitApp.md)
