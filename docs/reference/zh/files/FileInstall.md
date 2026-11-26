# `FileInstall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

像编译后脚本的安装器那样复制一个文件。这里不编译任何东西：只是复制文件。

## 语法

```ahk
FileInstall …
```

## 参数

Source, Dest, [Overwrite]

## 返回值

无返回值

## 错误

Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

## 示例

```ahk
FileInstall "data.txt", "data.out.txt"
```
