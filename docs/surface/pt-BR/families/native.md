# Chamadas a funções nativas (DllCall)

| [English](../index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Idioma: Português (Brasil)

Os nomes da família **Chamadas a funções nativas (DllCall)** nos três dialetos, com o que esta árvore responde por cada um. A família é a área em que um script pensa; os nomes são a totalidade dela.

| Nome | Dialeto | Tipo | Classe | Parâmetros | Estado | Sonda |
|---|---|---|---|---|---|---|
| `Buffer.__New` | AutoHotkey v2.0.19 | method | Buffer | -- | valor | -- |
| `Buffer` | AutoHotkey v2.0.19 | class | -- | -- | valor | -- |
| `DllCall` | AutoHotkey v2.0.19 | function | -- | up to 1 | valor | -- |
| `DllCall` | AutoHotkey v1.1.37.02 | function | -- | up to 1 | valor | -- |
| `LastError` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `NumGet` | AutoHotkey v2.0.19 | function | -- | 2 to 3 | valor | -- |
| `NumGet` | AutoHotkey v1.1.37.02 | function | -- | 1 to 3 | valor | -- |
| `NumPut` | AutoHotkey v2.0.19 | function | -- | up to 3 | valor | -- |
| `NumPut` | AutoHotkey v1.1.37.02 | function | -- | 2 to 4 | valor | -- |
| `ObjCreateWithDataPtr` | KeySharp | function | -- | -- | valor | -- |
| `ObjGetDataPtr` | KeySharp | function | -- | -- | valor | -- |
| `ObjGetDataSize` | KeySharp | function | -- | -- | valor | -- |
| `ObjSetDataPtr` | KeySharp | function | -- | -- | valor | -- |
| `Buffer.Ptr` | AutoHotkey v2.0.19 | method | Buffer | -- | valor | -- |
| `PtrSize` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `Buffer.Size` | AutoHotkey v2.0.19 | method | Buffer | -- | valor | -- |
| `VarSetStrCapacity` | AutoHotkey v2.0.19 | function | -- | 1 to 2 | valor | -- |

## O que a arvore faz com ele

### DllCall

Chama uma função de uma biblioteca da máquina; a conversão dos argumentos é pura e está na linguagem, a chamada é a capacidade do desktop, e a caixa responde com uma trampoline.

- **Parâmetros:** Target, Type1, Param1, [Type2, Param2, ...], [ReturnType]
- **Responde:** o valor que a função devolve
- **Erros:** "Failed to load DLL.", "Call to nonexistent function.", "Invalid arg type." and "Invalid return type." are the reasons of v2; a library of Windows on a Linux box is called through the raven, and a box without one says "this box has no raven: the libraries of Windows cannot be called."

```ahk
pid := DllCall("libc.so.6\getpid", "Int")
```

### DllCall

Chama uma função de uma biblioteca da máquina; a conversão dos argumentos é pura e está na linguagem, a chamada é a capacidade do desktop, e a caixa responde com uma trampoline.

- **Parâmetros:** Target, Type1, Param1, [Type2, Param2, ...], [ReturnType]
- **Responde:** o valor que a função devolve
- **Erros:** "Failed to load DLL.", "Call to nonexistent function.", "Invalid arg type." and "Invalid return type." are the reasons of v2; a library of Windows on a Linux box is called through the raven, and a box without one says "this box has no raven: the libraries of Windows cannot be called."

```ahk
pid := DllCall("libc.so.6\getpid", "Int")
```

### NumGet

Lê um número dos bytes de um buffer; a fonte de um número é um buffer, e a memória de um endereço não é lida aqui.

- **Parâmetros:** Buffer, [Offset], Type
- **Responde:** o número que os bytes guardam
- **Erros:** "this runtime does not read the memory of an address: the source of a number is a buffer." when the source is a number; a type that is not one of the eleven is refused with "the type of a number is one of UInt, Int, Int64, Short, UShort, Char, UChar, Double, Float, Ptr or UPtr."; bytes that do not fit are "the bytes do not fit in the buffer."

```ahk
value := NumGet(buf, 0, "Int")
```

### NumGet

Lê um número dos bytes de um buffer; a fonte de um número é um buffer, e a memória de um endereço não é lida aqui.

- **Parâmetros:** Buffer, [Offset], Type
- **Responde:** o número que os bytes guardam
- **Erros:** "this runtime does not read the memory of an address: the source of a number is a buffer." when the source is a number; a type that is not one of the eleven is refused with "the type of a number is one of UInt, Int, Int64, Short, UShort, Char, UChar, Double, Float, Ptr or UPtr."; bytes that do not fit are "the bytes do not fit in the buffer."

```ahk
value := NumGet(buf, 0, "Int")
```

### NumPut

Escreve números nos bytes de um buffer; os tipos e os números vêm em pares e o alvo vem depois deles, como o v2 escreve.

- **Parâmetros:** Type1, Number1, [Type2, Number2, ...], Target, [Offset]
- **Responde:** o endereço à direita do último número escrito
- **Erros:** "this runtime does not write to the memory of an address: the target of a number is a buffer." when the target is a number; a number of types that is not even is "the types and the numbers of a write come in pairs."; anything after the target and the offset is "a write takes the target and the offset after the pairs."

```ahk
NumPut("Int", 42, buf, 0)
```

### NumPut

Escreve números nos bytes de um buffer; os tipos e os números vêm em pares e o alvo vem depois deles, como o v2 escreve.

- **Parâmetros:** Type1, Number1, [Type2, Number2, ...], Target, [Offset]
- **Responde:** o endereço à direita do último número escrito
- **Erros:** "this runtime does not write to the memory of an address: the target of a number is a buffer." when the target is a number; a number of types that is not even is "the types and the numbers of a write come in pairs."; anything after the target and the offset is "a write takes the target and the offset after the pairs."

```ahk
NumPut("Int", 42, buf, 0)
```
