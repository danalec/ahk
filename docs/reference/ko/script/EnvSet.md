# `EnvSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파서와 스크립트 실행

## 설명

Sets an environment variable of the runner and answers the value; the script runs on one thread, so nothing else reads the environment while the change happens.

## 구문

```ahk
EnvSet Name, Value
```

## 매개변수

- **Name** — The variable.
- **Value** — The value; omitted deletes.

## 반환 값

the value that was set, as text

## 오류

an empty name and a name that carries an equal sign are refused with the reason of the machine.

## 예제

```ahk
EnvSet "MY_VAR", "value"
```

## 관련

[`EnvGet`](EnvGet.md)
