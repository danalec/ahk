# `IsSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*KeySharp* · function · 语言核心：文本、数学与模式

## 描述

调用者的变量是否有值。这个树除 `IsSetRef` 之外也带着它，它要的是变量，而不是变量的值。

## 语法

```ahk
IsSet 
```

## 参数

the variable, not its value

## 返回值

变量有值时为 1，没有时为 0

## 错误

an argument that is not a variable is refused with "IsSet requires a variable.".

## 示例

```ahk
x := 1
FileAppend IsSet(x), "*"
```
