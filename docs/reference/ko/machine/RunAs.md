# `RunAs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 머신, 프로세스, 환경

## 설명

Sets the user to run subsequent programs as.

## 구문

```ahk
RunAs User, Password, Domain
```

## 매개변수

- **User** — The user; omitted resets.
- **Password** — The password.
- **Domain** — The domain.

## 반환 값

Nothing.

## 예제

```ahk
RunAs "admin", "pw"
```

## 관련

[`Run`](Run.md), [`RunWait`](RunWait.md)
