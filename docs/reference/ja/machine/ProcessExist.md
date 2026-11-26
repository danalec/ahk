# `ProcessExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · マシン、プロセス、環境

## 説明

Answers whether a process is there, and with which pid; the processes of the machine are read out of /proc.

## 構文

```ahk
PID := ProcessExist(NameOrPID)
```

## パラメータ

- **NameOrPID** — The name or a PID.

## 戻り値

the pid of the process, or 0 when there is none

## エラー

a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none"; a process that is not there is zero and not a failure.

## 例

```ahk
if ProcessExist("firefox")
```

## 関連

[`ProcessClose`](ProcessClose.md), [`ProcessWait`](ProcessWait.md), [`Run`](Run.md)
