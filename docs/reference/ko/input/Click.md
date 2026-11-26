# `Click`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 핫키, 핫스트링, 입력

## 설명

Clicks with the pointer of the backend, which is an absolute uinput device: it moves to the place, settles, presses and releases.

## 구문

```ahk
Click Options
```

## 매개변수

- **Options** — X Y, button, count, Down/Up, Rel.

## 반환 값

nothing

## 오류

the wheel, the fourth and the fifth buttons, the pressing of a button and the place relative to the pointer are refused as not carried yet; on a host without `grim` the click fails with the reason of the screen, because the pointer needs the size of the screen.

## 설명

On Linux the absolute uinput pointer / XTEST.

## 예제

```ahk
Click 100, 200
```

## 관련

[`MouseMove`](MouseMove.md), [`MouseClick`](MouseClick.md), [`Send`](Send.md)
