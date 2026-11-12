# Registry APIs

| [English](index.md) | [Español](es/index.md) | [Português (Brasil)](pt-BR/index.md) | [日本語](ja/index.md) | [中文](zh/index.md) |

Language: English

The names of the family **Registry APIs** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State |
|---|---|---|---|---|---|
| `A_LoopRegName` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_RegView` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `LoopRegKey` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopRegName` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopRegTimeModified` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopRegType` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `RegCreateKey` | AutoHotkey v2.0.19 | function | -- | 0 to 1 | value |
| `RegDelete` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | value |
| `RegDelete` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `RegDeleteKey` | AutoHotkey v2.0.19 | function | -- | 0 to 1 | value |
| `RegRead` | AutoHotkey v2.0.19 | function | -- | 0 to 3 | value |
| `RegRead` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `RegView` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `RegWrite` | AutoHotkey v2.0.19 | function | -- | 0 to 4 | value |
| `RegWrite` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `SetRegView` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `SetRegView` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
