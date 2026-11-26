# `RunScript`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*KeySharp* · function · L'analyseur et l'exécution du script

## Description

Starts a script in a new process and answers a ScriptProcess (ks).

## Syntaxe

```ahk
P := RunScript(Path, Args)
```

## Paramètres

- **Path** — The script.
- **Args** — The arguments.

## Valeur de retour

A ScriptProcess.

## Exemple

```ahk
p := RunScript("child.ahk")
```

## Voir aussi

[`ScriptProcess`](ScriptProcess.md), [`Run`](Run.md)
