# `RegExReplace`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · 言語の中核: テキスト、数値、正規表現

## 説明

The text with the matches of a pattern replaced. The replacement may name the groups of the pattern with $1 or \1, and $$ is a dollar of its own.

## 構文

```ahk
RegExReplace …
```

## パラメータ

Text, Pattern, Replacement, [&Count], [Limit], [Start]

## 戻り値

the text with the replacements

## エラー

the count must be a variable reference; a pattern that does not compile fails with the reason of the compiler.

## 例

```ahk
RegExReplace("a1b2", "\d", "#")
```
