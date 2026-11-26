# `RunScript`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*KeySharp* · function · L'analizzatore e l'esecuzione dello script

## Descrizione

Starts a script in a new process and answers a ScriptProcess (ks).

## Sintassi

```ahk
P := RunScript(Path, Args)
```

## Parametri

- **Path** — The script.
- **Args** — The arguments.

## Valore di ritorno

A ScriptProcess.

## Esempio

```ahk
p := RunScript("child.ahk")
```

## Correlati

[`ScriptProcess`](ScriptProcess.md), [`Run`](Run.md)
