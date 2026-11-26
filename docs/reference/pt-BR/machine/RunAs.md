# `RunAs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · A máquina, os processos e o ambiente

## Descrição

Sets the user to run subsequent programs as.

## Sintaxe

```ahk
RunAs User, Password, Domain
```

## Parâmetros

- **User** — The user; omitted resets.
- **Password** — The password.
- **Domain** — The domain.

## Valor de retorno

Nothing.

## Exemplo

```ahk
RunAs "admin", "pw"
```

## Relacionados

[`Run`](Run.md), [`RunWait`](RunWait.md)
