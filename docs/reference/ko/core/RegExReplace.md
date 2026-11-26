# `RegExReplace`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

The text with the matches of a pattern replaced. The replacement may name the groups of the pattern with $1 or \1, and $$ is a dollar of its own.

## 구문

```ahk
RegExReplace …
```

## 매개변수

Text, Pattern, Replacement, [&Count], [Limit], [Start]

## 반환 값

the text with the replacements

## 오류

the count must be a variable reference; a pattern that does not compile fails with the reason of the compiler.

## 예제

```ahk
RegExReplace("a1b2", "\d", "#")
```
