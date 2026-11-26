# `Suspend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · L'analizzatore e l'esecuzione dello script

## Descrizione

Turns the hotkeys of the script off or on. With no parameter it turns them the other way. The state is visible in A_IsSuspended and in the enabled flag of the hotkey table.

## Sintassi

```ahk
Suspend …
```

## Parametri

[OnOff]

## Valore di ritorno

the empty text

## Errori

a parameter that is not 0, 1 or -1 is an invalid parameter.

## Esempio

```ahk
Suspend true
```
