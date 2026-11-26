# `Critical`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*KeySharp* · method · `Thread` · 解析器与脚本执行

## 描述

这棵树拒绝这个名字：这个运行时只用一个线程运行一个脚本，无法把它设为关键状态。

## 语法

```ahk
Critical 
```

## 参数

[OnOff]

## 返回值

无返回值；调用失败

## 错误

the refusal is "this runtime runs one script in one thread, and it cannot make it critical."

## 示例

```ahk
Critical
```
