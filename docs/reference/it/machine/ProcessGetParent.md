# `ProcessGetParent`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · command · La macchina, i processi e l'ambiente

## Descrizione

The parent PID of a process.

## Sintassi

```ahk
N := ProcessGetParent(NameOrPID)
```

## Parametri

- **NameOrPID** — The target.

## Valore di ritorno

The parent PID.

## Esempio

```ahk
pp := ProcessGetParent(pid)
```

## Correlati

[`ProcessExist`](ProcessExist.md)
