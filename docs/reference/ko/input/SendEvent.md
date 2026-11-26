# `SendEvent`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 핫키, 핫스트링, 입력

## 설명

Sends the keys the way the box sends them. The event method of v2 is the method of Windows, and the tree has one keyboard, so the keys go through it the way `Send` sends them.

## 구문

```ahk
SendEvent …
```

## 매개변수

Keys

## 반환 값

nothing

## 오류

the keyboard of the backend is a Linux uinput device, and it fails on a host that is not Linux.

## 예제

```ahk
SendEvent "^a"
```
