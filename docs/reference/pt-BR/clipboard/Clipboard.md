# `Clipboard`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · A área de transferência

## Descrição

A variável da área de transferência; o v2 a escreve A_Clipboard e o v1 a escreve Clipboard, e as duas são o mesmo texto. Lê-se com wl-paste numa sessão Wayland e com xclip ou xsel numa sessão X11, e escreve-se com wl-copy ou xclip.

## Sintaxe

```ahk
Clipboard 
```

## Parâmetros

--

## Valor de retorno

o texto da área de transferência

## Erros

an empty clipboard is a value and not a failure; an X11 session with neither xclip nor xsel answers "the clipboard of an X11 session cannot be read: neither xclip nor xsel is installed on this machine"; a write calls the function registered for OnClipboardChange with 1, because the change the runtime can see is its own.

## Exemplo

```ahk
text := A_Clipboard
```
