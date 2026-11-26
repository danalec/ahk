# `WinWaitActive`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Janelas da GUI e automação de janelas

## Descrição

Espera que uma janela cujo título combine tenha o foco, e devolve o identificador dela.

## Sintaxe

```ahk
WinWaitActive …
```

## Parâmetros

WinTitle, WinText, [Timeout], [ExcludeTitle], [ExcludeText]

## Valor de retorno

o identificador da janela como texto, ou 0 quando o tempo acabou

## Erros

a session that is not Wayland answers "this desktop waits for the windows of a Wayland session, and this session is not Wayland"; the two exclusions are refused with "the two exclusions are not carried yet."

## Exemplo

```ahk
WinWaitActive "Calculator", , 5
```
