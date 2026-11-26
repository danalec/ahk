# `ProcessSetPriority`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · Makine, süreçler ve ortam

## Açıklama

Sets the priority of a process.

## Sözdizimi

```ahk
ProcessSetPriority Priority, NameOrPID
```

## Parametreler

- **Priority** — L/B/N/A/H/R or a number.
- **NameOrPID** — The target.

## Dönüş değeri

Nothing.

## Örnek

```ahk
ProcessSetPriority "H", pid
```

## İlgili

[`Process`](Process.md)
