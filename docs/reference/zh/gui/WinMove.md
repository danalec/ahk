# `WinMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

移动并调整脚本自己创建的窗口的大小；Wayland 合成器自己摆放窗口，未必会照客户要求的坐标来。

## 语法

```ahk
WinMove X, Y, W, H, WinTitle
```

## 参数

- **X/Y/W/H** — New geometry; omitted keeps.
- **WinTitle** — The criteria.

## 返回值

空文本

## 错误

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## 示例

```ahk
WinMove 100, 100, 800, 600, "MyWindow"
```

## 相关

[`WinGetPos`](WinGetPos.md), [`WinMinimize`](WinMinimize.md)
