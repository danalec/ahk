# `GetKeySC`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 핫키, 핫스트링, 입력

## 설명

The code of the Linux input event of the key. It is the code the box writes to its own device, and not the scan code of the PC keyboard: the two numberings part company on the extended keys.

## 구문

```ahk
GetKeySC …
```

## 매개변수

KeyName

## 반환 값

the input event code, a number

## 오류

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## 예제

```ahk
sc := GetKeySC("Enter")
```
