# `GetKeyVK`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 핫키, 핫스트링, 입력

## 설명

The virtual key code of Windows of the key. It is the number of the key of the keyboard of Windows, and the tree keeps it for the scripts that ask for it.

## 구문

```ahk
GetKeyVK …
```

## 매개변수

KeyName

## 반환 값

the virtual key code, a number

## 오류

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## 예제

```ahk
vk := GetKeyVK("Enter")
```
