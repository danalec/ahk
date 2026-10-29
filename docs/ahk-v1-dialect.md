# O dialeto v1 ao lado do v2: o gate, as fases e o que fica de fora

> O plano. A autoridade é o binário do **AutoHotkey v1.1.37.02** e a
> documentação do v1 — medidos em 2026-10-28. A árvore fala **v2.0.19** hoje;
> este documento é sobre pôr o **v1** ao lado dele, sem mexer no v2.

---

## 0. O que este plano é, e o que ele inverte

A árvore é o v2.0.19. Este plano propõe um **segundo dialeto**, com o v1
encerrado atrás de um *gate*, e o v2 a ser o default.

Isto inverte uma linha escrita em `docs/ahk-x11-comparison.md`:

> | Não portar | Porquê |
> |---|---|
> | A sintaxe legacy v1 | O nosso dialeto é v2.0.19, e é essa a vantagem competitiva |

A razão continua válida para o **v2**: o v2 é o dialeto principal e a
vantagem competitiva. A mudança é que o v1 passa a ser uma **opção** —
para os scripts que já existem e que ninguém vai reescrever.

**A comparação está em revisão** — a onda de 2026-10-29 reescreve esse
documento, e os números das suas secções mudam com ele. A decisão está
escrita nos dois sítios: a comparação diz que a linha é verdade do v2 e
só do v2, e aponta para este plano; este plano diz o mesmo e aponta
para a comparação. E o `AGENTS.md` aponta para aqui. Duas outras linhas
da mesma tabela também ficam invertidas — para o dialecto do v1, e só
para ele:

| Linha da comparação | Para o v2 | Para o v1 |
|---|---|---|
| A convenção "output var" (`StringLen, out, in`) | o valor de retorno, e o `&` quando é preciso escrever | a convenção é a do v1 medido, e a tabela dos built-ins ganha a coluna da forma do parâmetro (§3.2) |
| `#NoEnv` como default | o ambiente é explícito, `EnvGet` está lá | o `#NoEnv` existe e desliga a substituição (§3) |

### A coordenação com a onda de 2026-10-29

Outra sessão trabalha nesta árvore ao mesmo tempo, e o trabalho dela é
**outro**: fechar as lacunas do port para dentro do **v2** — o clipboard,
o som, o `NumGet` e o `NumPut`, os ficheiros, a data — e construir o
comando `ahk --surface`, que imprime o estado de cada nome de v2. Está a
reescrever `docs/ahk-x11-comparison.md` ao mesmo tempo.

Os dois planos são **ortogonais**: a onda porta *capacidades* para
dentro do v2; este plano acrescenta um *dialecto* ao lado do v2, sem
mexer em nenhuma capacidade. Duas coisas ficam acordadas:

1. **O `--surface` é da onda, e este plano usa-o.** As fases F1 e F5
   precisam do estado de cada nome — e o comando já existe
   (`crates/ahk/src/surface.rs`, `ahk --surface`). Este plano
   **estende-o com um parâmetro**; não faz um segundo comando.
2. **A comparação é reescrita, e o plano não depende do número das
   secções.** O plano cita o texto; o `AGENTS.md` aponta para o plano.

A extensão que a F1 precisa é pequena. O `surface::entries()` de hoje lê
a tabela de v2 que está embutida e pergunta à árvore, nome a nome —
`builtins::find` e `builtins::refusal_of` para os comandos e as
funções, `variables::carries` e `variables::refusal_of` para as
variáveis, e `classes::carries` para os métodos. Com um dialecto, cada
uma dessas perguntas passa a levar o dialecto, e `entries()` passa a ser
`entries_for(dialect)`. O vocabulário fica como está — `Value`,
`Refusal(reason)`, `Pending(reason)`; é a frase do resumo que troca o
"de v2" pelo nome do dialecto.

E o `Pending` é o que a F1 precisa: a tabela de pendentes do v2 está
**vazia** — o port carrega o v2 inteiro — e a do v1 vai ser a primeira
que não está. É aí que entram os nomes do v1 que o v2 não tem, e o
`Pending` é a resposta certa para cada um deles.

### A autoridade

| Fonte | O que é | Onde |
|---|---|---|
| `AutoHotkeyU64.exe` (v1.1.37.02) | a verdade: as mensagens, as formas, os nomes | descarregado de `autohotkey.com/download/1.1/` a 2026-10-28; portátil, nada instalado |
| `autohotkey.com/docs/v1/` | a lista dos comandos e das funções, e a gramática |idem |
| `docs/ahk-x11-comparison.md` | os nomes que mudaram do v1 para o v2 (a secção "Os nomes que mudaram do v1 para o v2") | no repositório, em revisão |
| `AutoHotkey.chm` (no zip do v1.1) | a documentação offline | no zip |

**O arquivo `../archived/ahk-python` não serve de referência.** Não é uma
implementação do v1: a sua própria nota diz *"O dialeto é o do AutoHotkey
v2. `:=` atribui, `=` compara"* (`archived/ahk-python/AGENTS.md:38-41`), e
os seus 9 commits são todos de 2026-10-25 — o primeiro já era v2. É o
registo do dialecto *yoko-ahk* pré-port, não do v1.

### O que o gate tem de resolver

1. A **superfície**: os nomes, as aridades e as formas dos parâmetros. Os
   dois dialetos não têm os mesmos nomes, **nem o mesmo género para o
   mesmo nome** (ver §1.4).
2. A **gramática**: o comando legacy com vírgulas (`Msg, texto`), o `=`
   que atribui, os *labels*, o `if (x)` de uma só instrução, a hotstring
   como **instrução** e não como comando.
3. A **semântica**: o `=` do v1 compara sem distinção de maiúsculas, o
   `%nome%` é a *variável de ambiente*, as variáveis são globais por
   omissão, e o v1 tem *threads* — o v1.1 corre cada hotkey no seu
   thread, e o v2 não tem threads.

---

## 1. O que foi medido no v1.1.37.02 (2026-10-28)

Tudo o que segue foi medido no binário, com `/ErrorStdOut` (o v1 abre um
diálogo quando um script falha; com a opção, a razão sai no stdout e o
processo acaba).

### 1.1 As regras da sintaxe

| Regra | v1.1.37.02 medido | v2.0.19 (a árvore) |
|---|---|---|
| Comando legacy | `FileAppend, hello, C:\...\out.txt` escreve "hello" | `FileAppend "hello", "..."` — a vírgula depois do nome é erro de sintaxe (`parser.rs:462-499`) |
| Parâmetro de um comando | `StringLen, n, x` → `n` = 5: o parâmetro é o **nome de uma variável** | o parâmetro é uma expressão |
| Parâmetro de texto | `StringLen, n, %x%` → `n` = 0 (a forma `%…%` é a substituição *legacy*) | não há |
| Parâmetro de expressão | `StringLen, n, (x)` → recusado: *"The following variable name contains an illegal character: "(x)""* — a forma `(…)` só existe nos parâmetros que são expressão | toda a expressão |
| Atribuição | `x = 5` atribui (e `y := 7` também) | `:=` atribui; `=` compara |
| Comparação | `"ABC" = "abc"` → **verdade**; `"ABC" == "abc"` → falso; `"ABC" === "abc"` → falso | `=` é estrito, `==` é o que não distingue maiúsculas (`runtime.rs:2798-2809`) |
| `if` | `if (x > 1)` com **uma** instrução e `else` — sem chavetas | `if` com expressão e bloco `{ }` |
| *Labels* | `MyLabel:` e `goto MyLabel2` funcionam | não existem |
| Hotstring | é uma **instrução**: `::btw::by the way`, ou `::btw::` seguido das linhas e do `return` | `Hotstring(":btw:", Callback)` — um comando |
| Hotkey | `Hotkey, ^a, MyCallback` — o alvo é um *label* | `Hotkey("^a", Callback)` — o alvo é o objecto de uma função |
| Threads | `#MaxThreads 10`, `#MaxThreadsBuffer 100`, `Thread, Priority, 1` — todos existem | as directivas do v1 são recusadas (`runtime.rs:2131-2132`) |
| Directivas | `#NoEnv`, `#Persistent`, `#SingleInstance Force` existem | o conjunto é o do v2 (19) |
| Objectos | `o := {}`, `o.x := 5`, `Object()` — existem | `Map`, `Array`, `Buffer`, `Func`, `Gui`, `Menu`, `Hotkey` — nove classes, e o v1 não tem nenhuma |
| Erros | função desconhecida: *"Call to nonexistent function. Specifically: X"*; comando desconhecido: *"This line does not contain a recognized action. Specifically: X"* | *"Call to nonexistent function."* |

### 1.2 As variáveis do v1

`A_AhkVersion` é `1.1.37.02`. O v1 tem variáveis que o v2 não tem, e
valores que o v2 não partilha:

| Variável | v1.1.37.02 | v2.0.19 |
|---|---|---|
| `A_SendMode` | `Event` | `Input` |
| `A_StoreCapsLockMode` | `On` | `1` |
| `A_ControlDelay` | `20` | não existe |
| `A_WinDelay` | `100` | não existe |
| `A_DetectHiddenWindows` / `A_DetectHiddenText` | `Off` / `On` | não existem |
| `A_TitleMatchMode` | `1` | não existe |
| `A_FormatFloat` / `A_FormatInteger` | `0.6` / `D` | não existem |
| `A_IsAdmin`, `A_IsCompiled`, `A_IsCritical`, `A_IsPaused`, `A_IsSuspended` | existem | não existem |
| `A_IconFile`, `A_IconHidden`, `A_IconNumber`, `A_IconTip` | existem | não existem |
| `A_ToolTipDelay`, `A_ToolTipTimeout` | existem | não existem |
| `A_LineFile`, `A_LineNumber`, `A_LoopFile*`, `A_LoopReg*`, `A_LoopReadLine` | existem | alguns existem |
| `A_KeyDelay`, `A_MouseDelay`, `A_Now`, `A_NowUTC`, `A_WDay`, `A_YWeek`, `A_MMM`, `A_MMMM` | iguais | iguais |
| `A_CoordModeCaret`, `A_CoordModeMenu`, `A_FileEncoding` | não existem | existem |

### 1.3 A superfície: o que se mede, e como

Duas notícias, uma boa e uma má.

**A má: a extracção do v2 não se repete.** A superfície do v2 foi lida
**de dentro do binário**: as tabelas de nomes e de handlers. No v1 **os
nomes dos comandos não são texto simples no executável** — uma busca de
`WinActivate`, `SendMode`, `MouseMove`, `Click`, `MsgBox` e `Hotkey` por
bytes ASCII (e por maiúsculas) não encontra nada; `SetTimer` e
`BlockInput` aparecem só dentro das mensagens de ajuda. A tabela de
comandos do v1 está ofuscada no binário.

**A boa: a sondagem directa é barata e é a verdade.** No v1, uma função
desconhecida é um erro de **carga**, não de execução — o `try`/`catch` não
a apanha. A pergunta é então uma por processo:

```
x := StrLen("abc")     # existe: o ficheiro recebe "3"
x := Upper("abc")      # não existe: "Call to nonexistent function."
```

Quarenta e sete nomes medidos em 2026-10-28 levaram um minuto. Para a
superfície inteira do v1 (a do v2 tem **725** nomes — a tabela de
`crates/ahk/reference/ahk-v2.0.19.surface.tsv`, que a onda de
2026-10-29 já imprime com `ahk --surface`) são ~250 sondas: **dois
minutos** — na caixa, onde um comando perdido (um `Click` sem parâmetros
clica onde o rato estiver) não faz mal.

A sondagem tem uma utilidade a mais: o `--surface` da onda imprime o
estado de cada nome **de v2**; a mesma mecânica, com a tabela do v1 por
dentro, imprime o do v1 — e a sondagem é o que preenche essa tabela.

### 1.4 Os nomes: a maior parte coincide, o género não

Medido nos dois binários:

- **No v1 e não no v2**: `Asc`, `IsSet`, `Object`, `MenuGetName`,
  `Exception`.
- **No v2 e não no v1**: `StrUpper`, `StrLower` — *o v1.1.37.02 não tem
  nenhuma função de caixa*: nem `Upper`, nem `StrUpper`, nem `Lower`,
  nem `StrLower*. `Number`, `Transform`, `StringReplace` e `ComValue` também
  não são do v1.1 (os dois primeiros são de v1.0 ou do v2). `HasProp` e
  `HasMethod` também não — e o v1 não tem `ObjHasProp` tampoco (medido:
  *"Call to nonexistent function. Specifically: ObjHasProp(o, "a")"*).
- **O mesmo nome, outro género** — e é aqui que o gate dói:

| Nome | no v1.1.37.02 | no v2.0.19 |
|---|---|---|
| `StrSplit` | **função** | **comando** |
| `FileExist` | **função** | **comando** |
| `FileRead` | **comando** (com variável de saída) | **comando** |
| `GetKeyState` | **função** | **comando** |
| `StrReplace` | **função** (parâmetros por ordem diferente) | **função** |
| `Clipboard` | **não existe** | **variável** (o texto do clipboard) |
| `A_Clipboard` | **variável** (o texto) | não existe |
| `ClipboardAll` | **variável** (um objecto com todos os formatos) | não existe |
| `Length` do array | **método**: `a.Length()` | **propriedade**: `a.Length` |

O clipboard é o caso maisганho: no v1 o texto é `A_Clipboard` e o
objecto completo é `ClipboardAll`, e **não existe** nenhum nome
`Clipboard` — medido: `Clipboard, 0, hello` → *"This line does not
contain a recognized action. Specifically: Clipboard, 0, hello"*, e
`Clipboard()` → *"Call to nonexistent function"*. As duas variáveis do
v1 foram confirmadas com o `IsSet` do v1: `IsSet(A_Clipboard)` e
`IsSet(ClipboardAll)` dão 1, e um nome inventado dá 0.

Uma tabela por dialecto, com a coluna do género, é o que resolve isto.

### 1.5 As classes: o v1 não tem classes do runtime

O v2 tem onze classes na superfície — `Map`, `Array`, `Buffer`, `File`,
`Func`, `Gui` (com `Gui.Control`, `Gui.ListView`, `Gui.Menu` e
`Gui.StatusBar`) e `Hotkey` — e são **tipos do runtime**: o script não
as estende, e os métodos são os que o runtime responde.

O v1 não tem nenhum desses tipos. O que tem são **objectos**, e medido
no binário em 2026-10-29 o `.__Class` deles é **vazio**: o `File` de
`FileOpen`, o objecto de `Object()`, o literal `{a: 1}` e o objecto da
variável `ClipboardAll` dão todos `.__Class` vazio. O `Object` do v1
é uma **função** (`Object()` corre), e o objecto base não tem nome de
classe. O array do v1 também é um objecto: `a := [1, 2, 3]` dá
`a.Length()` = 3 (um **método**) e `a.__Class` vazio.

O v1 tem classes, mas são **do script**: `class`, `new`, `this` e
`__Init` existem — medido: `class Probe { __Init() { this.x := 1 } }`,
`new Probe`, e `o.x` vale 1 — e aí o `.__Class` é o nome da classe
("Probe"). O que o v1 não tem é uma *superfície* de classe por trás de
um nome: o `Map` e o `Array` do v2 são objectos com métodos; no v1 são
objectos com chaves.

---

## 2. O gate: três formas, e a que se escolhe

### (a) Feature de compilação — `--features v1`

Um binário por dialeto. Zero custo em runtime, e o v1 nem entra no
binário do v2. Em troca: trocar de dialeto é recompilar, o servidor do
lab precisa das duas construções, e a caixa tem de as guardar.

### (b) Sinalizador de runtime — `--dialect=v1` ← **a recomendada**

Um binário, os dois dialetos. O dialeto é um **valor** que o parser e o
runtime consultam; o v2 é o default e o caminho do v1 fica inerte para um
script v2. A caixa e o MCP mudam de dialeto sem recompilar.

Custa: um predicado no parser e outro no runtime, e as duas tabelas no
binário.

### (c) Crate separado — `ahk-v1`

A superfície do v1 fica isolada. Em troca: o runtime, o interpretador, as
classes e o `Desktop` duplicados e mantidos em paralelo — o dobro do
trabalho para sempre, e a árvore deixa de ter *uma* tabela e *um* dialecto.

### Porquê (b)

Três razões, na ordem do peso:

1. **O dialecto é uma propriedade do script, não do binário.** O lab
   corre scripts v1 e v2 no mesmo dia; com (a) seriam dois binários a
   manter na caixa.
2. **A superfície já é data.** `builtins.rs` é uma tabela; `variables.rs`
   é uma tabela; as classes são uma lista. Um segundo dialeto é,
   sobretudo, **um segundo conjunto de linhas** — e a tabela é
   exactamente o que a arquitectura da árvore já pede.
3. **O custo em runtime é um predicado.** O `Dialect` é um
   `&'static dyn Dialect`; o parser pergunta-o uma vez por instrução e o
   runtime uma vez por operação que dependa do dialecto.

A (b) tem um custo que a (a) não tem: **as duas gramáticas estão sempre
compiladas**. É o preço da gramática legacy (a instrução `Cmd, a, b` e a
hotstring-instrução `::btw::`), que é a peça mais cara do plano e que se
pagaria de qualquer maneira.

---

## 3. A forma do código

### 3.1 O trait

```rust
/// O dialecto que o runtime fala.
///
/// O v2 é o dialecto do Serialize: o `V2` é o que o runner usa por
/// omissão, e o `V1` só é escolhido quando o script o pede.
pub trait Dialect {
    /// O nome do dialecto, para o relatório.
    fn name(&self) -> &'static str;
    /// Os nomes que o dialecto carrega.
    fn builtins(&self) -> &'static [Builtin];
    /// As variáveis que o dialecto carrega.
    fn variables(&self) -> &'static [Variable];
    /// As classes de tipo que o dialecto carrega.
    fn classes(&self) -> &'static [&'static str];

    /// O comando legacy: `Msg, texto`.
    fn legacy_commands(&self) -> bool;
    /// O `=` atribui (e não compara).
    fn assign_with_equal(&self) -> bool;
    /// O `=` compara sem distinção de maiúsculas.
    fn equal_is_case_insensitive(&self) -> bool;
    /// O `%nome%` é a variável de ambiente.
    fn percent_is_environment(&self) -> bool;
    /// As variáveis são globais por omissão.
    fn scope_is_global(&self) -> bool;
    /// A hotstring é uma instrução: `::btw::`.
    fn hotstrings_are_statements(&self) -> bool;
}
```

O `Runtime` passa a carregar `dialect: &'static dyn Dialect` (ao lado do
`desktop`), e o `Parser` recebe-o no construtor. O `ahk-cli` ganha
`--dialect=v1|v2`, com o v2 por omissão.

### 3.2 A tabela dos built-ins ganha a forma dos parâmetros

O v1 tem três tipos de parâmetro: **variável** (o nome de uma variável
do script), **variável de saída** (o `n` de `StringLen, n, x`) e
**expressão**. O v2 tem dois: valor e referência. Uma coluna resolve os
dois:

```rust
/// A forma de um parâmetro.
pub enum Parameter {
    /// Uma expressão: o valor chega ao handler.
    Value,
    /// O nome de uma variável do script: o handler recebe a referência.
    Variable,
    /// O nome de uma variável do script onde o handler escreve.
    Output,
}
```

A linha de hoje é `Builtin { name, kind, min, max, variant, handler }` —
o `variant` é o tempero do handler (`Ceil` e `Floor` são o mesmo
handler com dois sabores). A coluna nova entra ao lado:
`parameters: &[Parameter]`. O `reference_parameters` actual —
`builtins.rs:457` — passa a ser lido dessa coluna. A tabela do v1
preenche a coluna com a forma que o v1 medido tiver.

### 3.3 O que **não** muda

O `Desktop` e os seus dez traits, o `ahk-box`, o relatório, os guard
rails, a classe `File`, o `DllCall`, o raven, a caixa. **O v1 corre sobre
o mesmo motor** — é o que torna o plano possível em vez de ser uma
reescrita.

### 3.4 As tabelas de referência

Passam a ser por dialecto, em `crates/ahk/reference/`:

```
ahk-v1.1.37.02.surface.tsv      # os nomes do v1, com o género e a aridade
ahk-v1.1.37.02.pending.tsv      # os nomes do v1 que a árvore não carrega
ahk-v2.0.19.surface.tsv         # o que já existe
ahk-v2.0.19.pending.tsv
```

E o teste de paridade (`tests/surface.rs`) passa a correr **contra o
dialecto escolhido**: para o v2 é o que é hoje; para o v1 é a mesma
mecânica com a outra tabela.

---

## 4. As fases

Cada fase tem um resultado **verificável**, e nenhuma delas mexe no
comportamento do v2.

### F0 — O gate, sem mudar nada

**Conteúdo.** O trait `Dialect`, o `V2` (que embrulha as tabelas de
hoje sem as mover), o parser e o runtime a perguntá-lo, o
`--dialect=v2` no runner, e a tabela dos parâmetros na tabela dos
built-ins (preenchida com `Value`/`Variable`, que é o que o v2 já faz).

**Resultado.** A suite continua verde sem mudar de corpo — os 622
casos do commit `6def58f` (a onda de 2026-10-29 também os mudou, e o
número só sobe); o `--dialect=v1` ainda responde *"this runtime does
not speak that dialect"*.

**Risco.** Baixo. É a fase que prova que o gate não custa nada ao v2.

### F1 — A superfície do v1 (dados)

**Conteúdo.** A tabela dos nomes do v1: a lista da documentação, o género
de cada um, a forma e a aridade de cada parâmetro — **verificadas no
binário**, com a sondagem de um processo por nome (§1.3), na caixa. As
variáveis do v1 (§1.2). As classes do v1: **nenhuma** (§1.5) — o v1
não tem classes do runtime, e o que ele tem são objectos e as classes
do script. A tabela de pendentes do v1, com a razão de cada nome que
não é carregado — e esta é a primeira tabela de pendentes que não está
vazia (§0).

**Resultado.** `ahk --surface --dialect=v1` imprime a lista com o estado
de cada nome — o comando que a onda de 2026-10-29 construiu para o v2,
com o dialecto a mais; o número de "carregados a sério" é visível em
cada commit.

**Risco.** Médio: é trabalho mecânico, mas é trabalho. E é a fase de que
depende a F2.

### F2 — A gramática do v1

**Conteúdo.**
- A instrução `Cmd, a, b` (o `parser.rs:462-499` é v2-only) e a
  desambiguação com a vírgula.
- A forma dos parâmetros: variável, variável de saída, expressão.
- O `=` que atribui (`parser.rs:353-361` e `:501-504` já leem `=` e `:=`
  para o mesmo braço — é quase metade do trabalho).
- O `if (x)` de uma instrução e o `else`.
- Os *labels* e o `goto`.
- A hotstring como instrução (`::btw::`).
- O `Hotkey, ^a, Label` com o alvo num label.
- As directivas do v1 (`#MaxThreads*`, `#NoEnv`, `#Persistent`,
  `#SingleInstance`, `#UseHook`, `#HotIf`, …).

**Resultado.** Um script v1 da documentação corre no runner: `MsgBox, text`
escreve a mensagem, `StringLen, n, s` preenche `n`, `if (a = b)` compara
sem distinção de maiúsculas, `goto` salta. (O `Msg` não é um comando do
v1 — medido em 2026-10-29: *"This line does not contain a recognized
action. Specifically: Msg, hello"*. O `MsgBox` é, e abre uma janela.)

**Risco.** Alto — é a fase mais cara. O `=` é o ponto difícil: o mesmo
token é atribuição ou comparação conforme a posição, e o v1 usa os dois
no mesmo sítio (`if (a = b)` compara; `a = b` atribui).

### F3 — A semântica do v1

**Conteúdo.**
- O `=` do v1 compara sem distinção de maiúsculas e o `==` distingue
  (o inverso do v2) — `runtime.rs:2798-2809`.
- O `%nome%` é a variável de ambiente, não a do script (o `#NoEnv` é o
  interruptor). **A árvore hoje interpola `%nome%` com a variável do
  script** (`runtime.rs:3073`) — é a regra do v1 com o sentido do v2, e
  o plano corrige-a por dialecto.
- As variáveis são globais por omissão (`runtime.rs:1648-1661` cria uma
  local); o v1 precisa de `global`.
- O texto é tudo: o v1 não tem inteiro e float separados, e o `Value` do
  runtime tem-nos (`runtime.rs:57-82`).
- As classes do v1: `Object` e o literal `{}`.

**Resultado.** O `=` e o `%nome%` de um script v1 comportam-se como o v1
medido, e um script v2 não sente nada.

**Risco.** Médio-alto: o modelo de valor e o de âmbito são estruturais.

### F4 — A caixa

**Conteúdo.** O v1 usa as mesmas capacidades, com os nomes do v1:
`Send`, `Click`, `PixelGet`, `WinExist`, `FileRead`, `FileAppend`,
`IniRead`, `StrReplace`, `StrLen`, `Format`, `RegExMatch`. As
capacidades que o v1 tem e o v2 não (`Control*`, `Win*`, o registo) são
recusadas com a razão, como o v2 faz.

O clipboard muda de forma, não de caminho: no v1 o texto é a variável
`A_Clipboard` e o objecto completo é a variável `ClipboardAll`; no v2 o
texto é a variável `Clipboard` e `ClipboardAll` não existe. O caminho é
o mesmo dos dois lados — o `wl-paste` e o `wl-copy` da caixa, e **não**
o protocolo do compositor (§27 das notas).

**Resultado.** Um script v1 que lê um pixel, escreve um ficheiro e manda
uma tecla funciona na caixa.

**Risco.** Baixo: é a camada do `Desktop`, que já sabe fazer quase tudo
isto.

### F5 — A verificação

**Conteúdo.**
- O teste de paridade por dialecto, com as duas tabelas.
- A suite do v1 (`crates/ahk/tests/v1/`) a espelhar a do v2.
- O `--surface` por dialecto, em texto e em JSON.
- A medição na caixa: os casos do v1 pelo runner, como os 35 do v2
  (2026-10-29).

**Resultado.** `ahk --surface --dialect=v1` diz o que falta, e o número
não piora sem se ver.

---

## 5. Os riscos

| Risco | Gravidade | O que se faz |
|---|---|---|
| **As threads do v1.** O v1 corre cada hotkey no seu thread, com `#MaxThreads` e `#MaxThreadsBuffer`; o v2 é cooperativo e tem guard rails. | **Alta** | Duas saídas: (a) o v1 corre no modelo cooperativo da árvore e `Thread`, `#MaxThreads*` e `SetTimer` com thread são recusados com a razão; (b) um modelo de threads real, que é outro projecto. **A (a) é a deste plano** — e é uma divergência que fica escrita. |
| **O `=` do v1.** Atribuição e comparação com o mesmo token. | Alta | O parser decide pela posição (a instrução começa com `=` → atribui; dentro de uma expressão → compara). Medido: `"ABC" = "abc"` é verdade. |
| **`%nome%` é a variável de ambiente no v1.** | Média | A árvore interpola a variável do script. Com o dialecto v1 passa a ser a de ambiente, e o `#NoEnv` desliga a substituição. É uma mudança de *sentido* de uma regra que a árvore já tem. |
| **A superfície não sai do binário.** Os nomes do v1 estão ofuscados. | Média | A lista vem da documentação e é verificada pelo binário, na caixa — §1.3. |
| **Duas tabelas para manter.** | Média | O teste de paridade por dialecto e o `--surface` tornam a deriva visível. |
| **A onda e o plano no mesmo `--surface`.** A onda de 2026-10-29 construiu o comando; este plano estende-o. | Média | A extensão é um parâmetro e quatro perguntas que passam a levar o dialecto. Não há um segundo comando, e o `AGENTS.md` diz qual é a peça de cada um. |
| **Duas gramáticas no mesmo parser.** | Média | A gramática legacy fica atrás do predicado do dialecto; o v2 não a vê. |
| **A licença do AHK_X11.** GPL-2.0. | Baixa | Já está escrito na comparação (a nota da licença): o *comportamento* pode ser portado, o código não. Este plano só usa a lista de nomes e o comportamento medido. |
| **O `v1` e o `v2` a divergirem com o tempo.** | Baixa | O gate é o dialecto, não a versão: uma tabela nova do v1 (1.1.37.03) é uma linha a mais. |

---

## 6. O que fica de fora

- **As threads do v1** (acima): o v1 corre no modelo cooperativo.
- **As classes do v2** (`Map`, `Array`, `Buffer`, `Func`, `Gui`, `Menu`,
  `Hotkey`) não existem no v1 — e o v1 não ganha classes novas.
- **O `Gui`, o `Menu` e o `TrayTip`**: no v1 são comandos e menus do
  sistema; a árvore recusa-os (o v2 também recusa o `Gui`).
- **O registo do Windows** (`RegRead`, `RegWrite`, `RegDelete`): recusado
  com a razão, como no v2.
- **O COM** (`ComObj`, `ComValue`): recusado.
- **O `Control*`**: recusado — a caixa não tem a camada de
  acessibilidade (ver `docs/wayland-toplevel-enumeration.md`).
- **As *variáveis de ambiente* como mecanismo**: o `#NoEnv` é aceite, e
  a substituição é a do v1.

---

## 7. Como se mede a paridade

A mecânica já existe dos dois lados:

- `crates/ahk/tests/surface.rs` lê a tabela da superfície e pergunta à
  árvore, nome a nome, se a carrega. Com o gate, a mesma pergunta corre
  para os dois dialetos.
- O **comando** também já existe: a onda de 2026-10-29 construiu o
  `ahk --surface` (`crates/ahk/src/surface.rs`), que imprime uma linha
  por nome e o resumo no fim. O gate não cria um segundo comando — dá-lhe
  um parâmetro:

```
ahk --surface --dialect=v1

WinActivate command refusal: this is a name of Windows, and this desktop is not Windows.
MsgBox command value
StringLen function pending: this runtime does not carry that name yet.
...
the surface of v1 has <n> entries: <v> answer with a value, <r> are refused, <p> are pending.
```

A forma é a de hoje: o nome, o género, e o estado — `value`,
`refusal: ...` ou `pending: ...`, com a razão quando a há. As três
linhas são o formato: um nome recusado com a razão, um nome que
responde, e um nome que o dialecto ainda não carrega. Os números do
resumo são o que interessa — e o do meio é o que diz o que falta. A
superfície do v1 ainda não foi contada: sai da sondagem da F1.

A linha de base é a do v2, no commit `0375057` (2026-10-29):

```
the surface of v2 has 725 entries: 457 answer with a value, 268 are refused, 0 are pending.
```

As 268 recusas são quase todas nomes de Windows — e a recusa é a
resposta certa para eles. A régua do plano é a do outro lado: nenhum
nome que o v1 faz e que a caixa consegue fazer devia estar na recusa por
falta de implementação.

O JSON, para o CI, é uma peça da F5: o comando de hoje imprime o texto.

O segundo número: **quantos nomes da superfície do v1 estão implementados
a sério**. A régua honesta é a mesma da comparação com o X11 — *nenhum
nome que o v1 faz e que a caixa consegue fazer devia estar na recusa por
falta de implementação*.

---

## 8. O resumo

| | |
|---|---|
| **O gate** | um valor: `--dialect=v1`, o v2 por omissão |
| **A forma** | o `Dialect` é um trait; as tabelas são dados; o parser e o runtime perguntam |
| **As fases** | F0 o gate · F1 a superfície · F2 a gramática · F3 a semântica · F4 a caixa · F5 a verificação |
| **A fase cara** | F2, a gramática legacy |
| **O que não muda** | o v2, o `Desktop`, a caixa, o relatório, os guard rails |
| **As classes do v1** | nenhuma: o v1 tem objectos e classes do script, não tipos do runtime |
| **A divergência que fica escrita** | o v1 corre no modelo cooperativo: sem threads |
| **A autoridade** | o binário do v1.1.37.02 e a documentação do v1 |
| **A dependência** | o `--surface` da onda de 2026-10-29, com o dialecto a mais |
| **A coordenação** | a onda porta capacidades para dentro do v2; este plano acrescenta o dialecto ao lado |
