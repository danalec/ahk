# `FileSetAttrib`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

修改文件的属性。参数顺序与 v2.0.31 的二进制一致：先是属性，后是文件名。文件名可以是模式，模式匹配的每个文件都会被修改。

## 语法

```ahk
FileSetAttrib …
```

## 参数

Attributes, FileName, [Recurse]

## 返回值

无返回值

## 错误

ValueError when the attributes are not one of v2; OSError when the attributes of the file cannot be changed. The third parameter is the recursion: the "R" walks into the subfolders, and any other letter is refused.

## 示例

```ahk
FileSetAttrib "+H", "*.txt"
```
