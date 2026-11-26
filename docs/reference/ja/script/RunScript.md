# `RunScript`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*KeySharp* · function · パーサとスクリプトの実行

## 説明

Starts a script in a new process and answers a ScriptProcess (ks).

## 構文

```ahk
P := RunScript(Path, Args)
```

## パラメータ

- **Path** — The script.
- **Args** — The arguments.

## 戻り値

A ScriptProcess.

## 例

```ahk
p := RunScript("child.ahk")
```

## 関連

[`ScriptProcess`](ScriptProcess.md), [`Run`](Run.md)
