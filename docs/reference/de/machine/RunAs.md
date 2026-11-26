# `RunAs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Die Maschine, die Prozesse und die Umgebung

## Beschreibung

Sets the user to run subsequent programs as.

## Syntax

```ahk
RunAs User, Password, Domain
```

## Parameter

- **User** — The user; omitted resets.
- **Password** — The password.
- **Domain** — The domain.

## Rückgabewert

Nothing.

## Beispiel

```ahk
RunAs "admin", "pw"
```

## Verwandt

[`Run`](Run.md), [`RunWait`](RunWait.md)
