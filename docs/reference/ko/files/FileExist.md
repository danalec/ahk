# `FileExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 파일과 디렉터리 작업

## 설명

Answers the attributes of the first entry that the name or the pattern matches. The value of the command is taken with the parenthesis, the way `FileExist("a.txt")` is written.

## 구문

```ahk
Attrib := FileExist(Path)
```

## 매개변수

- **Path** — A path or a wildcard pattern.

## 반환 값

the attributes as letters: `A` for a file, `D` for a folder, `RA` for a file that cannot be written, and the empty text when there is no such file.

## 오류

nothing fails: a name that matches nothing answers the empty text.

## 설명

A wildcard answers the first match in the folder's enumeration order.

## 예제

```ahk
x := FileExist("out.txt")
```

## 관련

[`DirExist`](DirExist.md), [`FileGetAttrib`](FileGetAttrib.md)
