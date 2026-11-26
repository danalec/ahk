# `StrGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言核心：文本、数学与模式

## 描述

缓冲区的文本，或某个地址处的文本。来源是缓冲区或数字，长度和编码在它之后。

## 语法

```ahk
StrGet …
```

## 参数

Source, [Length], [Encoding]

## 返回值

文本；既不对应缓冲区也不对应文本的地址返回空文本

## 错误

a text as the source is refused with "Parameter #1 invalid."; an encoding the dialect does not know is refused with "Invalid Encoding.".

## 示例

```ahk
StrGet(Buffer(16), 4, "UTF-8")
```
