# APIs do registro

| [English](../index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Idioma: Português (Brasil)

Os nomes da família **APIs do registro** nos três dialetos, com o que esta árvore responde por cada um. A família é a área em que um script pensa; os nomes são a totalidade dela.

| Nome | Dialeto | Tipo | Classe | Parâmetros | Estado | Sonda |
|---|---|---|---|---|---|---|
| `A_LoopRegName` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `A_RegView` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `LoopRegKey` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `LoopRegName` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `LoopRegTimeModified` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `LoopRegType` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `RegCreateKey` | AutoHotkey v2.0.19 | function | -- | 0 to 1 | valor | -- |
| `RegDelete` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | valor | -- |
| `RegDelete` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | recusado: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `RegDeleteKey` | AutoHotkey v2.0.19 | function | -- | 0 to 1 | valor | -- |
| `RegRead` | AutoHotkey v2.0.19 | function | -- | 0 to 3 | valor | -- |
| `RegRead` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | recusado: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `RegView` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `RegWrite` | AutoHotkey v2.0.19 | function | -- | 0 to 4 | valor | -- |
| `RegWrite` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | recusado: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `SetRegView` | AutoHotkey v2.0.19 | command | -- | 1 | valor | -- |
| `SetRegView` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | valor | -- |

## O que a arvore faz com ele

### A_LoopRegName

O nome da chave de registro em que o laço de registro está. O registro é uma coisa do Windows e este runtime não tem um: o laço de registro falha com "the registry is a Windows thing and this box has none.", e o nome continua o texto vazio.

- **Parâmetros:** nothing; it is a variable, not a call
- **Responde:** o texto vazio; a árvore nunca escreve esse nome
- **Erros:** nothing to read; the write is refused.

```ahk
FileAppend A_LoopRegName, "*"
```

### RegDelete

Remove um valor do registro do store da runtime; a chave em si fica.

- **Parâmetros:** KeyName, ValueName
- **Responde:** o texto vazio
- **Erros:** "Parameter #1 of RegDelete must not be omitted in this case." when the key is left out; a key without a hive of the ten is an invalid parameter; "(2) The system cannot find the file specified." when the key is not there.

```ahk
RegDelete "HKCU\Software\MyApp", "Setting"
```

### RegDelete

Remove um valor do registro do store da runtime; a chave em si fica.

- **Parâmetros:** KeyName, ValueName
- **Responde:** o texto vazio
- **Erros:** "Parameter #1 of RegDelete must not be omitted in this case." when the key is left out; a key without a hive of the ten is an invalid parameter; "(2) The system cannot find the file specified." when the key is not there.

```ahk
RegDelete "HKCU\Software\MyApp", "Setting"
```

### RegRead

Devolve o valor do registro; esta máquina não tem registro, e a árvore carrega a API sobre um store próprio: as hives são pastas e as chaves são arquivos, na pasta de configuração do usuário.

- **Parâmetros:** KeyName, ValueName, [Default]
- **Responde:** o valor, ou o padrão quando o valor não está lá
- **Erros:** "Parameter #1 of RegRead must not be omitted in this case." when the key is left out; "Parameter #1 of RegRead is invalid." when the name does not begin with a hive of the ten; "(2) The system cannot find the file specified." when the key or the value is not there and no default is given; a value with no name is "(161) The specified path is invalid."

```ahk
v := RegRead("HKCU\Software\MyApp", "Setting", "default")
```

### RegRead

Devolve o valor do registro; esta máquina não tem registro, e a árvore carrega a API sobre um store próprio: as hives são pastas e as chaves são arquivos, na pasta de configuração do usuário.

- **Parâmetros:** KeyName, ValueName, [Default]
- **Responde:** o valor, ou o padrão quando o valor não está lá
- **Erros:** "Parameter #1 of RegRead must not be omitted in this case." when the key is left out; "Parameter #1 of RegRead is invalid." when the name does not begin with a hive of the ten; "(2) The system cannot find the file specified." when the key or the value is not there and no default is given; a value with no name is "(161) The specified path is invalid."

```ahk
v := RegRead("HKCU\Software\MyApp", "Setting", "default")
```

### RegWrite

Escreve um valor no registro do store da runtime; a chave é criada quando não está lá, com os seus ancestrais.

- **Parâmetros:** Value, ValueType, KeyName, ValueName
- **Responde:** o texto vazio
- **Erros:** "Parameter #3 of RegWrite must not be omitted in this case." when the key is left out; "Parameter #2 of RegWrite is invalid." when the kind is not one of the five of the dialect; "(161) The specified path is invalid." when the name of the value is left out; a key without a hive is an invalid parameter.

```ahk
RegWrite "value", "REG_SZ", "HKCU\Software\MyApp", "Setting"
```

### RegWrite

Escreve um valor no registro do store da runtime; a chave é criada quando não está lá, com os seus ancestrais.

- **Parâmetros:** Value, ValueType, KeyName, ValueName
- **Responde:** o texto vazio
- **Erros:** "Parameter #3 of RegWrite must not be omitted in this case." when the key is left out; "Parameter #2 of RegWrite is invalid." when the kind is not one of the five of the dialect; "(161) The specified path is invalid." when the name of the value is left out; a key without a hive is an invalid parameter.

```ahk
RegWrite "value", "REG_SZ", "HKCU\Software\MyApp", "Setting"
```
