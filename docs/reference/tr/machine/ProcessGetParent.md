# `ProcessGetParent`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · Makine, süreçler ve ortam

## Açıklama

The parent PID of a process.

## Sözdizimi

```ahk
N := ProcessGetParent(NameOrPID)
```

## Parametreler

- **NameOrPID** — The target.

## Dönüş değeri

The parent PID.

## Örnek

```ahk
pp := ProcessGetParent(pid)
```

## İlgili

[`ProcessExist`](ProcessExist.md)
