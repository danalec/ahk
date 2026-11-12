# Registry APIs

| [English](index.md) | [Español](es/index.md) | [Português (Brasil)](pt-BR/index.md) | [Français](fr/index.md) | [Italiano](it/index.md) | [Polski](pl/index.md) | [Türkçe](tr/index.md) | [Deutsch](de/index.md) | [Русский](ru/index.md) | [日本語](ja/index.md) | [中文（简体）](zh/index.md) | [繁體中文](zh-Hant/index.md) | [한국어](ko/index.md) |

Language: English

The names of the family **Registry APIs** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State | Probe |
|---|---|---|---|---|---|---|
| `A_LoopRegName` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_RegView` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `LoopRegKey` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopRegName` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopRegTimeModified` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopRegType` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `RegCreateKey` | AutoHotkey v2.0.19 | function | -- | 0 to 1 | value | -- |
| `RegDelete` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | value | -- |
| `RegDelete` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `RegDeleteKey` | AutoHotkey v2.0.19 | function | -- | 0 to 1 | value | -- |
| `RegRead` | AutoHotkey v2.0.19 | function | -- | 0 to 3 | value | -- |
| `RegRead` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `RegView` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `RegWrite` | AutoHotkey v2.0.19 | function | -- | 0 to 4 | value | -- |
| `RegWrite` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `SetRegView` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `SetRegView` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value | -- |

## What the tree does with it

### A_LoopRegName

The name of the registry key the registry loop is on. The registry is a thing of Windows and this runtime has none: the registry loop fails with "the registry is a Windows thing and this box has none.", and the name keeps the empty text.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the empty text; the name is never written by the tree
- **Errors:** nothing to read; the write is refused.

```ahk
FileAppend A_LoopRegName, "*"
```

### RegDelete

Removes a value of the registry of the store of the runtime; the key itself stays.

- **Parameters:** KeyName, ValueName
- **Answers:** the empty text
- **Errors:** "Parameter #1 of RegDelete must not be omitted in this case." when the key is left out; a key without a hive of the ten is an invalid parameter; "(2) The system cannot find the file specified." when the key is not there.

```ahk
RegDelete "HKCU\Software\MyApp", "Setting"
```

### RegDelete

Removes a value of the registry of the store of the runtime; the key itself stays.

- **Parameters:** KeyName, ValueName
- **Answers:** the empty text
- **Errors:** "Parameter #1 of RegDelete must not be omitted in this case." when the key is left out; a key without a hive of the ten is an invalid parameter; "(2) The system cannot find the file specified." when the key is not there.

```ahk
RegDelete "HKCU\Software\MyApp", "Setting"
```

### RegRead

Answers the value of the registry; this machine has no registry, and the tree carries the API over a store of its own -- the hives are folders and the keys are files, under the config folder of the user.

- **Parameters:** KeyName, ValueName, [Default]
- **Answers:** the value of the value, or the default when the value is not there
- **Errors:** "Parameter #1 of RegRead must not be omitted in this case." when the key is left out; "Parameter #1 of RegRead is invalid." when the name does not begin with a hive of the ten; "(2) The system cannot find the file specified." when the key or the value is not there and no default is given; a value with no name is "(161) The specified path is invalid."

```ahk
v := RegRead("HKCU\Software\MyApp", "Setting", "default")
```

### RegRead

Answers the value of the registry; this machine has no registry, and the tree carries the API over a store of its own -- the hives are folders and the keys are files, under the config folder of the user.

- **Parameters:** KeyName, ValueName, [Default]
- **Answers:** the value of the value, or the default when the value is not there
- **Errors:** "Parameter #1 of RegRead must not be omitted in this case." when the key is left out; "Parameter #1 of RegRead is invalid." when the name does not begin with a hive of the ten; "(2) The system cannot find the file specified." when the key or the value is not there and no default is given; a value with no name is "(161) The specified path is invalid."

```ahk
v := RegRead("HKCU\Software\MyApp", "Setting", "default")
```

### RegWrite

Writes a value into the registry of the store of the runtime; the key is made when it is not there, with its ancestors.

- **Parameters:** Value, ValueType, KeyName, ValueName
- **Answers:** the empty text
- **Errors:** "Parameter #3 of RegWrite must not be omitted in this case." when the key is left out; "Parameter #2 of RegWrite is invalid." when the kind is not one of the five of the dialect; "(161) The specified path is invalid." when the name of the value is left out; a key without a hive is an invalid parameter.

```ahk
RegWrite "value", "REG_SZ", "HKCU\Software\MyApp", "Setting"
```

### RegWrite

Writes a value into the registry of the store of the runtime; the key is made when it is not there, with its ancestors.

- **Parameters:** Value, ValueType, KeyName, ValueName
- **Answers:** the empty text
- **Errors:** "Parameter #3 of RegWrite must not be omitted in this case." when the key is left out; "Parameter #2 of RegWrite is invalid." when the kind is not one of the five of the dialect; "(161) The specified path is invalid." when the name of the value is left out; a key without a hive is an invalid parameter.

```ahk
RegWrite "value", "REG_SZ", "HKCU\Software\MyApp", "Setting"
```
