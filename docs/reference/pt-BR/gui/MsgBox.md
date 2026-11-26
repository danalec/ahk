# `MsgBox`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Janelas da GUI e automação de janelas

## Descrição

Mostra uma mensagem. A árvore não tem janela própria, então o texto vai para a saída do relatório em vez de aparecer na tela, e a chamada devolve o mesmo texto.

## Sintaxe

```ahk
Result := MsgBox(Text, Title, Options)
```

## Parâmetros

- **Text** — The message. If omitted, an empty box.
- **Title** — The title of the window. Defaults to the script's name.
- **Options** — A number or a text of icons/buttons: icon (16/32/48/64), buttons (0..5), default button, modality, timeout, and the owner.

## Valor de retorno

o texto da mensagem

## Erros

more than one parameter is refused with "the options, the title and the variable that takes the answer are not carried yet."

## Observações

On the Linux box the tree has no window of its own in a headless run: the text goes to the report output and the call answers the text. On Windows it is the MessageBox of user32. A timeout answers Timeout.

## Exemplo

```ahk
MsgBox "hello"
```

## Relacionados

[`InputBox`](InputBox.md), [`ToolTip`](ToolTip.md), [`TrayTip`](TrayTip.md)
