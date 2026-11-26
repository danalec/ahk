# `DllCall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Chamadas a funções nativas (DllCall)

## Descrição

Chama uma função de uma biblioteca da máquina; a conversão dos argumentos é pura e está na linguagem, a chamada é a capacidade do desktop, e a caixa responde com uma trampoline.

## Sintaxe

```ahk
R := DllCall("Dll\Func", Args..., "CDecl ReturnType")
```

## Parâmetros

- **Dll\Func** — The library and symbol.
- **Args** — Type/value pairs.
- **ReturnType** — The return type.

## Valor de retorno

o valor que a função devolve

## Erros

"Failed to load DLL.", "Call to nonexistent function.", "Invalid arg type." and "Invalid return type." are the reasons of v2; a library of Windows on a Linux box is called through the raven, and a box without one says "this box has no raven: the libraries of Windows cannot be called."

## Observações

On Linux dlopen/dlsym; on Windows LoadLibraryW/GetProcAddress; refuses on macOS/FreeBSD.

## Exemplo

```ahk
pid := DllCall("libc.so.6\getpid", "Int")
```

## Relacionados

[`NumGet`](NumGet.md), [`Buffer`](Buffer.md), [`CallbackCreate`](CallbackCreate.md)
