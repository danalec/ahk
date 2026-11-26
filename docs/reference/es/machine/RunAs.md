# `RunAs`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · La máquina, los procesos y el entorno

## Descripción

Sets the user to run subsequent programs as.

## Sintaxis

```ahk
RunAs User, Password, Domain
```

## Parámetros

- **User** — The user; omitted resets.
- **Password** — The password.
- **Domain** — The domain.

## Valor de retorno

Nothing.

## Ejemplo

```ahk
RunAs "admin", "pw"
```

## Relacionados

[`Run`](Run.md), [`RunWait`](RunWait.md)
