# Registry APIs

The names of the family **Registry APIs** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State |
|---|---|---|---|---|---|
| `A_LoopRegName` | v1 | variable | -- | -- | value |
| `A_RegView` | v1 | variable | -- | -- | value |
| `LoopRegKey` | v2 | variable | -- | -- | value |
| `LoopRegName` | v2 | variable | -- | -- | value |
| `LoopRegTimeModified` | v2 | variable | -- | -- | value |
| `LoopRegType` | v2 | variable | -- | -- | value |
| `RegCreateKey` | v2 | function | -- | 0 to 1 | refused: the registry is a thing of Windows, and this runtime does not carry it. |
| `RegDelete` | v2 | function | -- | 0 to 2 | refused: the registry is a thing of Windows, and this runtime does not carry it. |
| `RegDelete` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `RegDeleteKey` | v2 | function | -- | 0 to 1 | refused: the registry is a thing of Windows, and this runtime does not carry it. |
| `RegRead` | v2 | function | -- | 0 to 3 | refused: the registry is a thing of Windows, and this runtime does not carry it. |
| `RegRead` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `RegView` | v2 | variable | -- | -- | value |
| `RegWrite` | v2 | function | -- | 0 to 4 | refused: the registry is a thing of Windows, and this runtime does not carry it. |
| `RegWrite` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `SetRegView` | v2 | command | -- | 1 | value |
| `SetRegView` | v1 | command | -- | up to 1 | value |
