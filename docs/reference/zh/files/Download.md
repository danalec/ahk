# `Download`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 文件与目录操作

## 描述

从网络上取一个文件并写到磁盘上。这个盒子用 `curl` 完成；v2 写在 url 前面的缓存标志会被接受并丢弃。

## 语法

```ahk
Download …
```

## 参数

URL, FileName

## 返回值

无返回值

## 错误

Error when the url cannot be reached at all, or when the machine has no `curl`; a file that is not there on the network is not a failure, and the page of the error of the server is what is written.

## 示例

```ahk
Download "https://example.com/x", "x.html"
```
