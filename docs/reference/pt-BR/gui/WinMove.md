# `WinMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Janelas da GUI e automação de janelas

## Descrição

Move e redimensiona a janela que o script criou; um compositor Wayland posiciona as janelas por conta própria e pode não aceitar o lugar pedido.

## Sintaxe

```ahk
WinMove X, Y, W, H, WinTitle
```

## Parâmetros

- **X/Y/W/H** — New geometry; omitted keeps.
- **WinTitle** — The criteria.

## Valor de retorno

o texto vazio

## Erros

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## Exemplo

```ahk
WinMove 100, 100, 800, 600, "MyWindow"
```

## Relacionados

[`WinGetPos`](WinGetPos.md), [`WinMinimize`](WinMinimize.md)
