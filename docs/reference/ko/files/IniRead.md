# `IniRead`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파일과 디렉터리 작업

## 설명

Reads the value of a key of an ini file. With no section it answers the names of the sections; with no key it answers the section itself. The ini files are the ones of Windows: `[section]`, `key=value`, and `;` for a comment.

## 구문

```ahk
V := IniRead(Filename, Section, Key, Default)
```

## 매개변수

- **Filename** — The INI.
- **Section** — The section.
- **Key** — The key; omitted reads the whole section.
- **Default** — Returned when absent.

## 반환 값

the value of the key; the default of the script when the key is not there and a default is given.

## 오류

Error `The requested key, section or file was not found.` when the key is not there and no default is given.

## 예제

```ahk
v := IniRead("app.ini", "Main", "Name")
```

## 관련

[`IniWrite`](IniWrite.md), [`IniDelete`](IniDelete.md)
