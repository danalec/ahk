# `EnvGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파서와 스크립트 실행

## 설명

Answers the value of an environment variable of the runner; the environment is the one of the process the script runs in.

## 구문

```ahk
V := EnvGet(Name)
```

## 매개변수

- **Name** — The variable.

## 반환 값

the value of the variable, or the empty text when the environment does not have it

## 오류

nothing; a name that is not there is the empty text and not a failure.

## 예제

```ahk
home := EnvGet("HOME")
```

## 관련

[`EnvSet`](EnvSet.md)
