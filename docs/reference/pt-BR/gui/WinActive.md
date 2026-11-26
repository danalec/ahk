# `WinActive`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Janelas da GUI e automação de janelas

## Descrição

A janela que tem o foco. O backend é uma sessão Wayland, e o compositor é quem sabe das janelas e qual delas está na frente.

## Sintaxe

```ahk
WinActive …
```

## Parâmetros

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Valor de retorno

o identificador da janela ativa, zero quando não há nenhuma

## Erros

the text of the window and the exclusions are not carried yet; a compositor that does not publish the focus answers with the reason, and not with a guess.

## Exemplo

```ahk
id := WinActive()
```
