# `WinMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · GUI 창과 창 자동화

## 설명

Moves and resizes the window the script made; a Wayland compositor places the windows itself and need not honour the place a client asks for.

## 구문

```ahk
WinMove X, Y, W, H, WinTitle
```

## 매개변수

- **X/Y/W/H** — New geometry; omitted keeps.
- **WinTitle** — The criteria.

## 반환 값

the empty text

## 오류

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## 예제

```ahk
WinMove 100, 100, 800, 600, "MyWindow"
```

## 관련

[`WinGetPos`](WinGetPos.md), [`WinMinimize`](WinMinimize.md)
