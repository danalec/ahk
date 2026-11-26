# `RunAs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Машина, процессы и окружение

## Описание

Sets the user to run subsequent programs as.

## Синтаксис

```ahk
RunAs User, Password, Domain
```

## Параметры

- **User** — The user; omitted resets.
- **Password** — The password.
- **Domain** — The domain.

## Возвращаемое значение

Nothing.

## Пример

```ahk
RunAs "admin", "pw"
```

## См. также

[`Run`](Run.md), [`RunWait`](RunWait.md)
