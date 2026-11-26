# `RunAs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · La machine, les processus et l'environnement

## Description

Sets the user to run subsequent programs as.

## Syntaxe

```ahk
RunAs User, Password, Domain
```

## Paramètres

- **User** — The user; omitted resets.
- **Password** — The password.
- **Domain** — The domain.

## Valeur de retour

Nothing.

## Exemple

```ahk
RunAs "admin", "pw"
```

## Voir aussi

[`Run`](Run.md), [`RunWait`](RunWait.md)
