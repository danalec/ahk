# `WinGetClientPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Janelas da GUI e automação de janelas

## Descrição

O mesmo lugar, da parte da janela em que o programa desenha. Um cliente Wayland não desenha a própria moldura -- quem desenha é o compositor -- então a geometria que o compositor relata é a janela que o usuário vê, e é a resposta dos dois comandos.

## Sintaxe

```ahk
WinGetClientPos …
```

## Parâmetros

[WindowTitle] [, WinText, &X, &Y, &Width, &Height, ExcludeTitle, ExcludeText]

## Valor de retorno

nada; o lugar e o tamanho são escritos nas variáveis de quem chama

## Erros

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## Exemplo

```ahk
WinGetClientPos "Untitled", , &x, &y, &w, &h
```
