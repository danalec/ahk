# `RegExReplace`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言核心：文本、数学与模式

## 描述

把模式的匹配替换掉之后的文本。替换文本可以用 $1 或 \1 引用模式的分组，$$ 表示一个美元符号本身。

## 语法

```ahk
RegExReplace …
```

## 参数

Text, Pattern, Replacement, [&Count], [Limit], [Start]

## 返回值

替换之后的文本

## 错误

the count must be a variable reference; a pattern that does not compile fails with the reason of the compiler.

## 示例

```ahk
RegExReplace("a1b2", "\d", "#")
```
