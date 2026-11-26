# `InputBox`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Janelas da GUI e automação de janelas

## Descrição

Pede um texto ao usuário numa caixa da área de trabalho e devolve um objeto com o texto em Value e o modo de fechamento em Result.

## Sintaxe

```ahk
Result := InputBox(Prompt, Title, Options, Default)
```

## Parâmetros

- **Prompt** — The question.
- **Title** — The window title.
- **Options** — Password/size/timeout.
- **Default** — The initial text.

## Valor de retorno

um objeto com as propriedades Value e Result, onde Result é "OK", "Cancel" ou "Timeout"

## Erros

an option the dialect does not know is refused with "Invalid option.".

## Exemplo

```ahk
answer := InputBox("your name?", "hello")
MsgBox answer.Value
```

## Relacionados

[`MsgBox`](MsgBox.md), [`FileSelect`](FileSelect.md)
