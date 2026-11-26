# `WinActivate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Janelas da GUI e automação de janelas

## Descrição

Traz uma janela para a frente. Age na janela do próprio script: a listagem do compositor não publica um jeito de erguer a janela de outro programa.

## Sintaxe

```ahk
WinActivate WinTitle, WinText, ExcludeTitle, ExcludeText
```

## Parâmetros

- **WinTitle** — Title/ahk_class/ahk_exe/ahk_id/ahk_pid.
- **WinText** — A text the window must contain.
- **Exclude*** — Criteria to skip.

## Valor de retorno

nada

## Erros

a window that is not there answers 'Target window not found.'; a window of another program cannot be activated this way.

## Observações

On Wayland the compositor may refuse focus; the tree reports the reason.

## Exemplo

```ahk
WinActivate "Untitled"
```

## Relacionados

[`WinWait`](WinWait.md), [`WinActive`](WinActive.md), [`WinClose`](WinClose.md)
