# `Run`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 机器、进程与环境

## 描述

运行一条命令并等待它结束。命令写到输出上的内容进入报告的输出。

## 语法

```ahk
Run Target, WorkingDir, Options, OutputVarPID
```

## 参数

- **Target** — A program, document, URL or verb.
- **WorkingDir** — The working directory of the child.
- **Options** — Min/Max/Hide/UseErrorLevel.
- **OutputVarPID** — A variable that receives the process id.

## 返回值

命令的退出码

## 错误

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## 备注

On Linux it is sh -c; on Windows ShellExecute/CreateProcess. The child outlives the script unless killed.

## 示例

```ahk
Run "ls"
```

## 相关

[`RunWait`](RunWait.md), [`Process`](Process.md), [`WinWait`](WinWait.md)
