# `RunAs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 機器、行程與環境

## 描述

Sets the user to run subsequent programs as.

## 語法

```ahk
RunAs User, Password, Domain
```

## 參數

- **User** — The user; omitted resets.
- **Password** — The password.
- **Domain** — The domain.

## 傳回值

Nothing.

## 範例

```ahk
RunAs "admin", "pw"
```

## 相關

[`Run`](Run.md), [`RunWait`](RunWait.md)
