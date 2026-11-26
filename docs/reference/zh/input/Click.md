# `Click`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 热键、热字符串与输入

## 描述

用后端的指针点击，该指针是绝对的 uinput 设备：先移动到位、停顿，再按下并松开。

## 语法

```ahk
Click Options
```

## 参数

- **Options** — X Y, button, count, Down/Up, Rel.

## 返回值

无返回值

## 错误

the wheel, the fourth and the fifth buttons, the pressing of a button and the place relative to the pointer are refused as not carried yet; on a host without `grim` the click fails with the reason of the screen, because the pointer needs the size of the screen.

## 备注

On Linux the absolute uinput pointer / XTEST.

## 示例

```ahk
Click 100, 200
```

## 相关

[`MouseMove`](MouseMove.md), [`MouseClick`](MouseClick.md), [`Send`](Send.md)
