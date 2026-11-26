# `RunAs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Makine, süreçler ve ortam

## Açıklama

Sets the user to run subsequent programs as.

## Sözdizimi

```ahk
RunAs User, Password, Domain
```

## Parametreler

- **User** — The user; omitted resets.
- **Password** — The password.
- **Domain** — The domain.

## Dönüş değeri

Nothing.

## Örnek

```ahk
RunAs "admin", "pw"
```

## İlgili

[`Run`](Run.md), [`RunWait`](RunWait.md)
