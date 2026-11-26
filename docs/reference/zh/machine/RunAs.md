# `RunAs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 机器、进程与环境

## 描述

Sets the user to run subsequent programs as.

## 语法

```ahk
RunAs User, Password, Domain
```

## 参数

- **User** — The user; omitted resets.
- **Password** — The password.
- **Domain** — The domain.

## 返回值

Nothing.

## 示例

```ahk
RunAs "admin", "pw"
```

## 相关

[`Run`](Run.md), [`RunWait`](RunWait.md)
