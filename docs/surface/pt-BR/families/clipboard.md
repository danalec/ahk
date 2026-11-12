# A área de transferência

| [English](../index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Idioma: Português (Brasil)

Os nomes da família **A área de transferência** nos três dialetos, com o que esta árvore responde por cada um. A família é a área em que um script pensa; os nomes são a totalidade dela.

| Nome | Dialeto | Tipo | Classe | Parâmetros | Estado | Sonda |
|---|---|---|---|---|---|---|
| `#ClipboardTimeout` | AutoHotkey v1.1.37.02 | directive | -- | -- | valor | -- |
| `A_Clipboard` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `Clipboard` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `Clipboard` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `ClipboardAll` | AutoHotkey v2.0.19 | class | -- | -- | valor | -- |
| `OnClipboardChange` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | valor | -- |
| `ClipboardAll.Ptr` | AutoHotkey v2.0.19 | method | ClipboardAll | -- | valor | -- |
| `ClipboardAll.Size` | AutoHotkey v2.0.19 | method | ClipboardAll | -- | valor | -- |

## O que a arvore faz com ele

### A_Clipboard

O texto da área de transferência da área de trabalho. Um script pode escrevê-la, e a concatenação soma ao que já está lá.

- **Parâmetros:** nothing; it is a variable, not a call
- **Responde:** o texto da área de transferência
- **Erros:** the read fails with the reason of the desktop; the write of a number is refused, because the clipboard holds a text.

```ahk
A_Clipboard := "hello"
FileAppend A_Clipboard, "*"
```

### Clipboard

A variável da área de transferência; o v2 a escreve A_Clipboard e o v1 a escreve Clipboard, e as duas são o mesmo texto. Lê-se com wl-paste numa sessão Wayland e com xclip ou xsel numa sessão X11, e escreve-se com wl-copy ou xclip.

- **Parâmetros:** --
- **Responde:** o texto da área de transferência
- **Erros:** an empty clipboard is a value and not a failure; an X11 session with neither xclip nor xsel answers "the clipboard of an X11 session cannot be read: neither xclip nor xsel is installed on this machine"; a write calls the function registered for OnClipboardChange with 1, because the change the runtime can see is its own.

```ahk
text := A_Clipboard
```
