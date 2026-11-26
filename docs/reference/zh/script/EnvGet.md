# `EnvGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 解析器与脚本执行

## 描述

给出运行器环境变量的值；环境就是脚本所在进程的环境。

## 语法

```ahk
V := EnvGet(Name)
```

## 参数

- **Name** — The variable.

## 返回值

变量的值，环境里没有时是空文本

## 错误

nothing; a name that is not there is the empty text and not a failure.

## 示例

```ahk
home := EnvGet("HOME")
```

## 相关

[`EnvSet`](EnvSet.md)
