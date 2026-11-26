# `WinGetClass`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Janelas da GUI e automação de janelas

## Descrição

A classe da janela que casa com o título dado.

## Sintaxe

```ahk
WinGetClass …
```

## Parâmetros

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Valor de retorno

a classe da janela

## Erros

a title that matches no window answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## Exemplo

```ahk
class := WinGetClass("Untitled")
```
