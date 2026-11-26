# `IniWrite`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파일과 디렉터리 작업

## 설명

Writes a key of an ini file. The value comes first and the name of the file second, the way the binary of v2.0.31 takes them; a file that is not there is made.

## 구문

```ahk
IniWrite Value, Filename, Section, Key
```

## 매개변수

- **Value** — The text.
- **Filename** — The INI.
- **Section** — The section.
- **Key** — The key.

## 반환 값

nothing

## 오류

Error when the file cannot be written.

## 예제

```ahk
IniWrite "hello", "app.ini", "Main", "Name"
```

## 관련

[`IniRead`](IniRead.md), [`IniDelete`](IniDelete.md)
