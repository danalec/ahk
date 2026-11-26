# `CoordMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 解析器与脚本执行

## 描述

说明坐标如何解读。v2 有五个目标：鼠标、像素、插入符、菜单和提示；本树没有窗口，插入符、菜单和提示的模式照样保存，但没有人使用它们。

## 语法

```ahk
CoordMode …
```

## 参数

Target [, Relative]

## 返回值

之前的模式

## 错误

a target or a relative that v2 does not know is refused as an invalid parameter.

## 示例

```ahk
CoordMode "Mouse", "Screen"
```
