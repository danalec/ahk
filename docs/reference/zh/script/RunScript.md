# `RunScript`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*KeySharp* · function · 解析器与脚本执行

## 描述

Starts a script in a new process and answers a ScriptProcess (ks).

## 语法

```ahk
P := RunScript(Path, Args)
```

## 参数

- **Path** — The script.
- **Args** — The arguments.

## 返回值

A ScriptProcess.

## 示例

```ahk
p := RunScript("child.ahk")
```

## 相关

[`ScriptProcess`](ScriptProcess.md), [`Run`](Run.md)
