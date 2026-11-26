# `Sort`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*KeySharp* · method · `Array` · 语言核心：文本、数学与模式

## 描述

文本中各条目排好序之后的文本。第二个参数是选项，逐字母读取；第三个参数是比较两个条目的回调。

## 语法

```ahk
S := Sort(String, Options)
```

## 参数

- **String** — The lines.
- **Options** — N (numeric), R (reverse), D delim, C case, etc.

## 返回值

条目排序之后的文本

## 错误

the first parameter must be a text; an array is refused; a third parameter that cannot be called fails with "This value of type ... has no method named Call".

## 示例

```ahk
Sort("b`na`n", "N")
```

## 相关

[`StrSplit`](StrSplit.md), [`Array`](Array.md)
