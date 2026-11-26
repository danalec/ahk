# `RawWrite`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · method · `File` · 文件与目录操作

## 描述

Writes raw bytes from a Buffer/variable.

## 语法

```ahk
N := File.RawWrite(Var, Bytes)
```

## 参数

?.

## 返回值

The bytes written.

## 示例

```ahk
n := f.RawWrite(buf, 16)
```

## 相关

[`File`](File.md), [`RawRead`](RawRead.md)
