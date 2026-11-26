# `FileOpen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 文件与目录操作

## 描述

打开文件并返回它的对象（`File` 类）。模式有 `r`、`w`、`a`、`rw`；文件名 `*` 是机器的控制台。

## 语法

```ahk
File := FileOpen(Filename, Flags, Encoding)
```

## 参数

- **Filename** — The path; * is the console.
- **Flags** — r/w/a/rw plus share mode.
- **Encoding** — UTF-8/UTF-16/CPnnn/RAW.

## 返回值

一个 `File` 对象。

## 错误

ValueError when the mode is not one of the four; OSError when the file cannot be opened.

## 备注

The File object keeps the handle; Close releases it.

## 示例

```ahk
f := FileOpen("out.txt", "w")
```

## 相关

[`FileRead`](FileRead.md), [`FileAppend`](FileAppend.md)
