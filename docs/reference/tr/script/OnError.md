# `OnError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Ayrıştırıcı ve betiğin çalıştırılması

## Açıklama

Registers the function that handles a failure the script did not catch.

## Sözdizimi

```ahk
OnError Function, Add
```

## Parametreler

- **Function** — The callback (Thrown, Mode).
- **Add** — 1/-1.

## Dönüş değeri

the integer 1

## Hatalar

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## Örnek

```ahk
OnError((err, mode) => 0)
```

## İlgili

[`OnExit`](OnExit.md), [`Throw`](Throw.md)
