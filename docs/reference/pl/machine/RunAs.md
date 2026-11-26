# `RunAs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Maszyna, procesy i środowisko

## Opis

Sets the user to run subsequent programs as.

## Składnia

```ahk
RunAs User, Password, Domain
```

## Parametry

- **User** — The user; omitted resets.
- **Password** — The password.
- **Domain** — The domain.

## Wartość zwracana

Nothing.

## Przykład

```ahk
RunAs "admin", "pw"
```

## Powiązane

[`Run`](Run.md), [`RunWait`](RunWait.md)
