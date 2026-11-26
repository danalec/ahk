# `Run`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 機器、行程與環境

## 描述

Runs a command and waits for it. What the command wrote on its output goes into the output of the report.

## 語法

```ahk
Run Target, WorkingDir, Options, OutputVarPID
```

## 參數

- **Target** — A program, document, URL or verb.
- **WorkingDir** — The working directory of the child.
- **Options** — Min/Max/Hide/UseErrorLevel.
- **OutputVarPID** — A variable that receives the process id.

## 傳回值

the exit code of the command

## 錯誤

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## 備註

On Linux it is sh -c; on Windows ShellExecute/CreateProcess. The child outlives the script unless killed.

## 範例

```ahk
Run "ls"
```

## 相關

[`RunWait`](RunWait.md), [`Process`](Process.md), [`WinWait`](WinWait.md)
