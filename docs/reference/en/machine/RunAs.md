# `RunAs`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · The machine, the processes and the environment

## Description

Sets the user to run subsequent programs as.

## Syntax

```ahk
RunAs User, Password, Domain
```

## Parameters

- **User** — The user; omitted resets.
- **Password** — The password.
- **Domain** — The domain.

## Return value

Nothing.

## Example

```ahk
RunAs "admin", "pw"
```

## Related

[`Run`](Run.md), [`RunWait`](RunWait.md)
