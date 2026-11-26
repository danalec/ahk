# `BlockInput`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 핫키, 핫스트링, 입력

## 설명

Turns the input of the whole machine off, and that is all it can do. Turning input off for the whole machine is a privilege this compositor does not give a client, and this tree will not claim it did.

## 구문

```ahk
BlockInput …
```

## 매개변수

Mode

## 반환 값

nothing for 'Off'

## 오류

any other mode fails with 'blocking the input of the whole machine is a privilege this compositor does not give a client'.

## 예제

```ahk
BlockInput "Off"
```
