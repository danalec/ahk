# `EnvSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 解析器与脚本执行

## 描述

设置运行器的一个环境变量并返回该值；脚本只有一个线程，改动时没有别的代码在读环境。

## 语法

```ahk
EnvSet Name, Value
```

## 参数

- **Name** — The variable.
- **Value** — The value; omitted deletes.

## 返回值

被设置的值（文本）

## 错误

an empty name and a name that carries an equal sign are refused with the reason of the machine.

## 示例

```ahk
EnvSet "MY_VAR", "value"
```

## 相关

[`EnvGet`](EnvGet.md)
