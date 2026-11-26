# `BlockInput`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 热键、热字符串与输入

## 描述

只能关闭整机的输入。整机屏蔽输入是本合成器不授予客户端的权限，本树不会谎称做到了。

## 语法

```ahk
BlockInput …
```

## 参数

Mode

## 返回值

'Off' 时无返回值

## 错误

any other mode fails with 'blocking the input of the whole machine is a privilege this compositor does not give a client'.

## 示例

```ahk
BlockInput "Off"
```
