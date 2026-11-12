# Native function calls (DllCall)

| [English](index.md) | [Español](es/index.md) | [Português (Brasil)](pt-BR/index.md) | [Français](fr/index.md) | [Italiano](it/index.md) | [Polski](pl/index.md) | [Türkçe](tr/index.md) | [Deutsch](de/index.md) | [Русский](ru/index.md) | [日本語](ja/index.md) | [中文（简体）](zh/index.md) | [繁體中文](zh-Hant/index.md) | [한국어](ko/index.md) |

Language: English

The names of the family **Native function calls (DllCall)** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State | Probe |
|---|---|---|---|---|---|---|
| `Buffer.__New` | AutoHotkey v2.0.19 | method | Buffer | -- | value | -- |
| `Buffer` | AutoHotkey v2.0.19 | class | -- | -- | value | -- |
| `DllCall` | AutoHotkey v2.0.19 | function | -- | up to 1 | value | -- |
| `DllCall` | AutoHotkey v1.1.37.02 | function | -- | up to 1 | value | -- |
| `LastError` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `NumGet` | AutoHotkey v2.0.19 | function | -- | 2 to 3 | value | -- |
| `NumGet` | AutoHotkey v1.1.37.02 | function | -- | 1 to 3 | value | -- |
| `NumPut` | AutoHotkey v2.0.19 | function | -- | up to 3 | value | -- |
| `NumPut` | AutoHotkey v1.1.37.02 | function | -- | 2 to 4 | value | -- |
| `ObjCreateWithDataPtr` | KeySharp | function | -- | -- | value | -- |
| `ObjGetDataPtr` | KeySharp | function | -- | -- | value | -- |
| `ObjGetDataSize` | KeySharp | function | -- | -- | value | -- |
| `ObjSetDataPtr` | KeySharp | function | -- | -- | value | -- |
| `Buffer.Ptr` | AutoHotkey v2.0.19 | method | Buffer | -- | value | -- |
| `PtrSize` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `Buffer.Size` | AutoHotkey v2.0.19 | method | Buffer | -- | value | -- |
| `VarSetStrCapacity` | AutoHotkey v2.0.19 | function | -- | 1 to 2 | value | -- |

## What the tree does with it

### DllCall

Calls a function of a library of the machine; the marshalling is pure and in the language crate, the call is the capability of the desktop, and the box answers it with a trampoline.

- **Parameters:** Target, Type1, Param1, [Type2, Param2, ...], [ReturnType]
- **Answers:** the value the function answers
- **Errors:** "Failed to load DLL.", "Call to nonexistent function.", "Invalid arg type." and "Invalid return type." are the reasons of v2; a library of Windows on a Linux box is called through the raven, and a box without one says "this box has no raven: the libraries of Windows cannot be called."

```ahk
pid := DllCall("libc.so.6\getpid", "Int")
```

### NumGet

Reads a number out of the bytes of a buffer; the source of a number is a buffer, and the memory of an address is not read here.

- **Parameters:** Buffer, [Offset], Type
- **Answers:** the number the bytes hold
- **Errors:** "this runtime does not read the memory of an address: the source of a number is a buffer." when the source is a number; a type that is not one of the eleven is refused with "the type of a number is one of UInt, Int, Int64, Short, UShort, Char, UChar, Double, Float, Ptr or UPtr."; bytes that do not fit are "the bytes do not fit in the buffer."

```ahk
value := NumGet(buf, 0, "Int")
```

### NumPut

Writes numbers into the bytes of a buffer; the types and the numbers come in pairs and the target comes after them, as v2 writes it.

- **Parameters:** Type1, Number1, [Type2, Number2, ...], Target, [Offset]
- **Answers:** the address to the right of the last number that was written
- **Errors:** "this runtime does not write to the memory of an address: the target of a number is a buffer." when the target is a number; a number of types that is not even is "the types and the numbers of a write come in pairs."; anything after the target and the offset is "a write takes the target and the offset after the pairs."

```ahk
NumPut("Int", 42, buf, 0)
```
