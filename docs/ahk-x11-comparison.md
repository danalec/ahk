# AHK_X11 × AHK — o que o AHK_X11 já fez, o que o nosso tree já faz, e o que falta

> Documento de comparação e de sugestões.
>
> - **AHK_X11** — <https://phil294.github.io/AHK_X11/> e
>   <https://github.com/phil294/AHK_X11>. Reimplementação do AutoHotkey para
>   Linux, escrita em Crystal, por Philip Waritschlager, **GPL-2.0**.
> - **AHK (este repositório)** — o AutoHotkey **v2.0.19** em Rust, em três
>   crates (`ahk`, `ahk-box`, `ahk-cli`), **Apache 2.0**.
>
> As fontes de cada afirmação estão no fim do documento. O que é **medido** na
> caixa (192.168.0.40) está marcado com a data; o que é **lido** de código
> alheio está marcado como tal; o que é **inferido** está marcado como
> inferência.

---

## 0. Como ler este documento

O AHK_X11 e o nosso tree não são a mesma coisa, e a comparação só faz sentido
depois de dizer **o que cada um é**:

| | AHK_X11 | AHK (nosso) |
|---|---|---|
| Objectivo | Trazer o AutoHotkey **v1.0.24** (2004) para Linux/X11 | Trazer o AutoHotkey **v2.0.19** (2024) para Linux/Wayland, com portabilidade |
| Dialeto | **Legacy v1**: `x = 1`, `MsgBox, texto`, `%var%`, pseudo-arrays | **v2.0.19**: `x := 1`, `MsgBox texto`, classes, objetos, métodos, `&` |
| Linguagem | Crystal 1.5.1 (binário único) | Rust (3 crates, núcleo `no_std`) |
| Superfície | 226 comandos de v1 — 127 feitos (56%), 15 novos, 11 removidos, 70 por fazer | 725 entradas da superfície de v2.0.19 — **93 built-ins com implementação real**, 253 com recusa motivada |
| Janelas | X11 completo (xdotool fork): activação, mover, dimensionar, minimizar, fechar | Wayland: **recusa motivada** — o compositor não expõe lista de janelas |
| Injecção | XTEST (via `x_do`) | **uinput** (teclado e ponteiro próprios) + `grim` para o ecrã |
| GUI | GTK3 (Janelas, `MsgBox`, `InputBox`, `Menu`, `ToolTip`, `TrayTip`) | `Gui`/`Menu`/`Hotkey` **recusados** com a razão |
| Classes | Nenhuma (v1 não tem) | `Map`, `Array`, `Buffer`, `File`, `Func`, `Gui`, `Menu`, `Hotkey`, `Object` |
| Concorrência | Threads cooperativas, interrompíveis (`src/run/thread.cr`) | Um script, um fio; guard rails em vez de interrupção |
| Licença | GPL-2.0 | Apache 2.0 |

**A conclusão de uma linha**: o AHK_X11 é *antigo no dialecto e completo no
ecrã*; nós somos *novos no dialecto e incompletos no ecrã*. As sugestões da
seção 5 exploitam exactamente essa divisão.

---

## 1. O AHK_X11 em uma página

### 1.1 Ficha técnica

- **Repositório**: <https://github.com/phil294/AHK_X11> — branch `master`,
  Crystal 1.5.1, `shard.yml` na versão **1.0.7**. A documentação descreve o
  dialecto do AutoHotkey **v1.0.24**, que é o que ele implementa.
- **Autoria**: Philip Waritschlager, GPL-2.0. A documentação original do
  AutoHotkey é de Chris Mallett (AutoIt).
- **Dependências** (todas Crystal, nenhuma de runtime externo para o núcleo):
  `x11` (fork `phil294/x11-cr`), `xtst` (extensão XTEST), `x_do` (fork do
  xdotool), `gtk3`, `atspi` (acessibilidade), `notify` (notificações), `tasker`
  (linha de comandos).
- **Empacotamento**: binário único; `ahk_x11.AppImage`; `shard` publica o
  pacote AUR (`ahk_x11`, `ahk_x11-bin`, `ahk_x11-git`); instalador gráfico;
  ficheiros `.desktop` e `mime.xml` para associar `.ahk`; compilador para
  executável standalone (`--compile`).
- **Modos de execução**: `ahk_x11 script.ahk`, `ahk_x11 < script.ahk` (stdin),
  `--repl` (consola interativa), `--compile`, `--windowspy`, shebang
  (`#!/caminho/ahk_x11`), e `DISPLAY=` para correr *headless* (sem GUI).
- **Requisito duro**: **X11**. Wayland não é suportado (issue #2 do
  projecto). Ubuntu 22.04+ e Wayland por omissão são o caso difícil.

### 1.2 Números oficiais (README do projecto)

```
DONE      56% (127/226)
NEW       7%  (15/226)
REMOVED   5%  (11/226)
TO DO     31% (70/226)
```

O README diz também: *"Currently about 80% of work of getting there is done"*,
com o objectivo declarado de 80% de paridade funcional.

### 1.3 O que ele é, e o que ele não é

É uma **reimplementação do AHK v1.0.24**, completa o suficiente para correr
scripts reais, mas sem as invenções modernas do AHK:

- **Sem expressões** — só a sintaxe legacy (`x = %y%`, `MsgBox, texto`).
- **Sem classes nem objectos** — `class MyClass {}` não existe; só variáveis.
- **Sem arrays de verdade** — pseudo-arrays com `StringSplit` +
  `StringTrimLeft`, exactamente como a documentação original fazia.
- **Sem functions** — `Gosub` e etiquetas; `#DefineCommand` é o substituto
  improvisado.
- **`#NoEnv` é o default** — o ambiente não é injectado nas variáveis.
- **Todos os argumentos são avaliados em runtime**, mesmo os estáticos.

Isto é, em si mesmo, a **maior lição** do AHK_X11 para nós: ele prova que a
parte difícil de um AHK para Linux não é a linguagem, é o **ecrã**. E prova
também o **custo** de ficar preso a um dialecto morto: metade do esforço
gasto em compatibilidade v1 é esforço que a v2 resolve de outra maneira.

---

## 2. As implementações do AHK_X11, por área

Esta é a lista completa do que o projecto faz, lida do código e da
documentação. Cada linha diz **o mecanismo** (como ele faz) porque é isso
que interessa quando se decide o que portar.

### 2.1 O runtime

- Binário Crystal único; execução por script, por stdin, por REPL, ou
  compilado para executável standalone.
- Parser próprio (`src/build/parser.cr`) e *linker* (`src/build/linker.cr`)
  que embute o script num executável novo.
- `src/run/runner.cr` (~15 KB) coordena: **display**, **threads**, **definições**
  do runner (a "last found window", `detect_hidden_windows`, o pid do som).
- `src/run/display/` tem **adaptadores** (`display-adapter.cr`) com uma
  interface abstracta de 6 métodos: `run`, `grab_hotkey`, `ungrab_hotkey`,
  `block_input`, `unblock_input`. As implementações são `x11.cr` (18 KB) e
  `gtk.cr` (17 KB).
- `src/cmd/` tem um comando por ficheiro, ~120 ficheiros, agrupados por
  família (`control-flow`, `file`, `gtk`, `math`, `misc`, `string`, `unix`,
  `x11`).

### 2.2 Hotkeys e hotstrings

- **Hotkeys** (`src/run/display/hotkey.cr`, 5 KB): `XGrabKey` directo sobre o
  servidor X, com os modificadores e as opções do AHK v1 (`t` — transparent,
  `c` — *context-sensitive*, `p` — *physical*, `i` — *ignore all modifiers*,
  `w` — *wildcard*).
- **Remap de teclas** — `F1::LButton` funciona; um botão do rato acionado
  por outro botão não pode ser reenviado (limitação do X11, documentada).
- **Hotstrings** (`src/run/display/hotstring.cr`, 2 KB): motor de reconhecimento
  de palavra, com expansões `::btw::By the way`, `:R:` (literal), `:C:` etc.
- **`BlockInput`** (`src/cmd/x11/block-input.cr`): bloqueia a entrada do
  utilizador.
- **`Pause`**, **`Suspend`**, **`Reload`**: o script pode ser pausado,
  suspenso ou recarregado.
- **Teclas capturadas**: `src/run/display/pressed-keys.cr`.

### 2.3 Envio de teclas

- **`Send`** (`src/cmd/x11/keyboard/send.cr`, 4,8 KB) e **`SendRaw`**: injectam
  por **XTEST**, com o parser de chaves do AHK (`{Ctrl}`, `{F1}`, `{Blind}`…).
- **`Send` é limitado pelo layout**: `§` não sai; a nota do README é usar
  `SendRaw`.
- **`Input`** (`src/cmd/x11/keyboard/input.cr`, 3 KB): lê o que o utilizador
  escreve, com `EndKeys` e `MatchList`.
- **`KeyWait`**, **`GetKeyState`**, **`SetKeyDelay`**.

### 2.4 Rato

- **`MouseMove`, `MouseClick`, `MouseClickDrag`, `MouseGetPos`,
  `SetMouseDelay`** (`src/cmd/x11/mouse/`), por XTEST.
- **`ControlClick`** (`src/cmd/x11/mouse/control-click.cr`): clique num
  controlo, via posição + AT-SPI.
- **`Click`** é um comando *novo* (v2-like) que o projecto acrescenta.

### 2.5 Janelas

Este é o subsystema mais completo do AHK_X11, e vale a pena ler
`src/cmd/x11/window/win-util.cr` (4,5 KB) com atenção:

- **Busca de janelas** pelo `x_do` (fork do xdotool): por nome (regex), por
  classe (`ahk_class`), por `ahk_id`, pelo desktop actual, e só as visíveis
  quando `DetectHiddenWindows` está off.
- **`WinText` / `ExcludeText`** são resolvidos por **AT-SPI**, não por X11.
- **Comandos**: `WinActivate`, `WinActivateBottom` (por fazer), `WinClose`,
  `WinGet` (`id`, `pid`, `processname`, `controllist`, `count`, `list`),
  `WinGetPos`, `WinGetText`, `WinGetTitle`, `WinGetClass`, `WinHide`,
  `WinKill`, `WinMaximize`, `WinMinimize`, `WinMinimizeAll`/`Undo`,
  `WinMove`, `WinRestore`, `WinSet`, `WinSetTitle`, `WinShow`, `WinWait`,
  `WinWaitActive`, `WinWaitClose`, `WinWaitNotActive`.
- **`IfWinExist` / `IfWinNotExist` / `IfWinActive` / `IfWinNotActive`**.
- **A "last found window"** — conceito do v1 que o v2 abandonou.

### 2.6 Controlos — a camada de acessibilidade

O ficheiro `src/run/display/at-spi.cr` tem **20 KB** e é o mais substancial
fora do núcleo X11. Ele implementa:

- **`ControlGetText`**, **`ControlSetText`**, **`ControlGetPos`**
  (`src/cmd/x11/control/`), **`ControlSend` / `ControlSendRaw`**.
- Leitura do texto de uma janela inteira, para o `WinGetText` e para o
  `WinText`.
- **`ControlClick`** (posição + AT-SPI).

**Esta é a peça mais valiosa do AHK_X11 para nós** — ver a seção 5.4. O
AT-SPI é um protocolo **D-Bus**, não um protocolo X11: funciona em Wayland.

### 2.7 Ecrã

- **`PixelGetColor`** (`src/cmd/misc/pixel-get-color.cr`) e
  **`PixelSearch`** (`src/cmd/misc/pixel-search.cr`): leem a imagem do ecrã por
  `XGetImage` e procuram a cor.
- **`CoordMode`**: `Screen` ou `Relative` para rato, pixel e tooltip.
- **`Screenshot`** não existe (é nome do v2 — nós temos, e é a única
  invenção do nosso dialeto; ver AGENTS.md §14).

### 2.8 GUI e diálogos (GTK3)

- **`Gui`** completo (`src/cmd/gtk/gui/`, 7,8 KB só no `gui-add.cr`):
  `Add` com ~30 tipos de controlo (`Text`, `Edit`, `Button`, `ComboBox`,
  `ListView`, `TreeView`, `Tab`, `Picture`, `Progress`, `Slider`, `DateTime`,
  `MonthCal`, `GroupBox`, `Link`, `Hotkey`, `DDL`, `UpDown`…), `Show`, `Hide`,
  `Submit`, `Cancel`, `Destroy`, `Color`, `Option`, `Scroll`.
- **`GuiControl`**: `Add`, `Show`, `Hide`, `Enable`, `Disable`, `Focus`,
  `Choose`, `ChooseString`, `ChooseIndex`.
- **Diálogos**: **`MsgBox`** (com os botões e os ícones do AHK v1),
  **`InputBox`**, **`FileSelectFile`**, **`FileSelectFolder`**,
  **`IfMsgBox`**.
- **`Menu`** (`src/cmd/gtk/menu.cr`): **só o menu da bandeja** — o comando recusa
  qualquer nome que não seja `TRAY`. Sub-comandos: `Add` (que também cria o
  ícone), `Icon`, `NoIcon`, `Tip`.
- **`ToolTip`** (`src/cmd/gtk/tooltip.cr`) e **`TrayTip`**
  (libnotify).

### 2.9 Ficheiros

- `FileAppend`, `FileCopy`, `FileCreateDir`, `FileDelete`, `FileGetSize`,
  `FileRead`, `FileReadLine`, `FileRecycle` (mete no lixo),
  `FileRecycleEmpty`, `FileSetAttrib`, `FileSetTime`, `FileGetAttrib`.
- `IniRead`, `IniWrite`, `IniDelete`.
- `SplitPath`, `SetWorkingDir`, `DriveSpaceFree`.
- `Loop Files` (com `R` recursivo), `Loop Read`, `Loop Parse`.

### 2.10 Som

- **`SoundPlay`** (`src/cmd/misc/sound-play.cr`): uma escada de reprodutores,
  por ordem de preferência — `ffplay` → `cvlc` → `paplay` → `mpg123` →
  `aplay`. Mata o anterior antes de tocar.
- **`SoundGet` / `SoundSet`** (`src/cmd/misc/sound-get.cr`): o volume sai de
  `amixer` (ALSA), não do PulseAudio.
- **`SoundGetWaveVolume` / `SoundSetWaveVolume`**: por fazer.

### 2.11 Processos e ambiente

- **`Run` / `RunWait`** (`src/cmd/misc/run.cr`, 6,8 KB), **`RunAs`**,
  **`Shutdown`**, **`URLDownloadToFile`**, **`Edit`**.
- **`EnvGet`** (comando *novo* no AHK_X11), **`EnvSet`**, `EnvAdd`, `EnvSub`,
  `EnvMult`, `EnvDiv`.

### 2.12 `DllCall` — a ponte para as librarias do sistema

`src/cmd/misc/dll-call.cr` (3,6 KB) usa o **FFI do Crystal**:
`Crystal::Loader` (que é `dlopen`/`dlsym`) e `Crystal::FFI::CallInterface`.
Tipos: `Int`, `Int64`, `Short`, `Char`, `Float`, `Double`, `Str`, `Ptr`, com
sufixo `P`/`*` para endereço. O `Str` é lido de volta como texto.

Limitações declaradas no código: as variáveis do AHK_X11 são **só texto**, e
cada valor não-`Ptr` é serializado e deserializado — "a lot of conversion
going on which isn't optimal but necessary due to ahkx11's string-only vars".

### 2.13 Sombreamento em binário

Três comandos que o AHK_X11 acrescenta para manipular memória:

- **`NumGet`** (`src/cmd/misc/num-get.cr`): lê um número de um buffer.
- **`NumPut`** (`src/cmd/misc/num-put.cr`): escreve um número num buffer.
- **`VarSetCapacity`** (`src/cmd/misc/var-set-capacity.cr`): dimensiona o
  buffer de uma variável.
- A base é `src/util/ahk-bin-var.cr`, que mapeia entre o texto da variável e
  os bytes.

### 2.14 Threads — o modelo v1

`src/run/thread.cr` (9,2 KB) implementa as threads **cooperativas** do AHK v1:

- Uma thread nova **interrompe** a corrente; as interrompidas ficam dormentes
  e são retomadas por ordem inversa.
- Cada thread tem as suas próprias definições (ErrorLevel, SetKeyDelay).
- **Prioridades** (`Thread`, `SetTimer` com prioridade): uma thread de
  prioridade menor não interrompe a corrente.
- `OnExit` corre sempre, independentemente da prioridade.

### 2.15 Empacotamento e ferramentas

- **Compilador** (`src/build/`): script → executável standalone, com o
  *parser* e o *linker* próprios; o binário gerado tem hotkeys e GUIs.
- **Window Spy** (`src/window-spy.ahk`): mostra a janela e o controlo sob o
  rato.
- **Instalador gráfico** (`src/installer.ahk`, 5,8 KB) + `ahk_x11-compiler.desktop`,
  `ahk_x11-windowspy.desktop`, `ahk_x11.desktop`, `ahk_x11-mime.xml`.
- **REPL**: depois do auto-execute, aceita comandos de uma linha; cada linha
  corre na sua thread.
- **`Echo`** (`src/cmd/unix/echo.cr`): imprime no stdout — o "modo Unix".
- **`Eval`** (`src/cmd/misc/eval.cr`): avalia código em runtime.

### 2.16 Os comandos que o AHK_X11 acrescenta (15)

`Echo`, `Eval`, `#DefineCommand`, `DllCall`, `NumGet`, `NumPut`,
`VarSetCapacity`, `FileRead`, `RegExGetPos`, `RegExReplace`, `EnvGet`,
`Click`, `ahk_x11_print_vars`, `ahk_x11_track_performance_start`,
`ahk_x11_track_performance_stop`.

### 2.17 O que o AHK_X11 remove (11)

`EnvUpdate`, `PostMessage`, `RegDelete`, `RegRead`, `RegWrite`, `SendMessage`,
`#InstallKeybdHook`, `#InstallMouseHook`, `#UseHook`, `Loop (registry)` —
*"those that simply make no sense in Linux"*.

`AutoTrim` foi posto de parte por uma razão que vale a pena copiar: *"It's
always Off. It would not differentiate between %a_space% and %some_var%. It's
possible but needs significant work."*

### 2.18 O que fica por fazer (70)

As famílias: `Drive*`, `FileCopyDir`/`FileMove`/`FileMoveDir`/`FileRemoveDir`,
`FileGetShortcut`/`FileCreateShortcut`/`FileGetVersion`/`FileInstall`,
`FileGetTime`/`FileGetAttrib`, `FormatTime`, `SetFormat`, `StringCaseSense`,
`Process`, `SysGet`, `Thread` (o comando, não o modelo), `SetBatchLines`,
`OnExit`, `Progress`/`SplashImage`/`SplashTextOn`, `GuiControlGet`,
`GroupAdd`/`GroupActivate`/`GroupClose`/`GroupDeactivate`,
`StatusBarGetText`/`StatusBarWait`, `WinMenuSelectItem`,
`Control`/`ControlFocus`/`ControlGet`/`ControlGetFocus`/`ControlMove`,
`DetectHiddenText`, `SetTitleMatchMode`, `SetWinDelay`, `SetControlDelay`,
`SetDefaultMouseSpeed`, `SetNumScrollCapsLockState`/`SetStoreCapslockMode`,
`KeyHistory`, `ListHotkeys`/`ListLines`/`ListVars`, `SoundGetWaveVolume`/
`SoundSetWaveVolume`, `FormatTime`, e as directivas `#CommentFlag`,
`#ErrorStdOut`, `#EscapeChar`, `#HotkeyInterval`, `#HotkeyModifierTimeout`,
`#MaxHotkeysPerInterval`, `#MaxMem`, `#MaxThreads`, `#MaxThreadsBuffer`,
`#MaxThreadsPerHotkey`, `#WinActivateForce`.

---

## 3. O nosso tree, pela mesma lista

### 3.1 A arquitectura

```
ahk-cli  (runner)  →  ahk-box (primitivas do Linux)  →  ahk (linguagem)
```

- **`ahk`** — lexer, AST, parser, runtime. **Não faz I/O.** Compila sem
  `std` (`cargo build -p ahk --no-default-features --target thumbv7em-none-eabihf`).
  A linguagem é **v2.0.19**, lida do binário: 725 entradas de superfície
  (644 nomes distintos), 11 tabelas de classe, tabela de operadores.
- **`ahk-box`** — a caixa: teclado uinput, ponteiro uinput, `grim`, `xprop`,
  `dlopen`, o *raven*, ficheiros.
- **`ahk-cli`** — o runner: script no stdin, relatório JSON no stdout, ficheiro
  de pid para a paragem de emergência.

O runtime é entregue a um `Desktop`, que é a soma de **dez** capacidades:
`Keyboard`, `Pointer`, `Screen`, `Windows`, `Processes`, `Files`, `Clipboard`,
`Timers`, `Hotkeys`, `Native`. Não há implementação por omissão: cada
implementador escreve as dez.

### 3.2 O que responde com valor, e o que responde com a razão

A tabela de built-ins (`crates/ahk/src/builtins.rs`) tem **346 entradas**. A
medição de hoje:

| Estado | Quantos | O que é |
|---|---|---|
| Handler a sério | **93** | O comando trabalha |
| `not_carried` | **253** | O nome existe e responde **com a razão** |

As cinco razões de `not_carried`:

| Variante | Razão | Nomes típicos |
|---|---|---|
| 0 | *"this is a name of Windows, and this desktop is not Windows."* | `WinMove`, `ControlClick`, `ToolTip`, `SoundPlay`, `MenuSelect` |
| 1 | *"this runtime has no hotkeys yet."* | `Hotkey`, `Hotstring`, `HotIf`, `HotIfWinActive` |
| 2 | *"the clock of this runtime counts from an arbitrary origin, and it does not know the date."* | `FormatTime`, `DateAdd`, `DateDiff` |
| 3 | *"this runtime is one script that ends when it ends, and it cannot do that."* | `Persistent`, `Reload`, `Suspend`, `Pause`, `OnExit` |
| 4 | *"this desktop cannot serve that with the files it has."* | `FileCopy`, `FileMove`, `FileGetSize`, `IniRead`, `IniWrite`, `DirCopy` |

Os 93 com implementação real, por família: matemática (`Abs`, `Ceil`,
`Floor`, `Round`, `Max`, `Min`, `Mod`, `Sqrt`, `Exp`, `Ln`, `Log`, `Sin`,
`Cos`, `Tan`, `ASin`, `ACos`, `ATan`, `Random`), texto (`Chr`, `Ord`, `Format`,
`StrLen`, `StrUpper`, `StrLower`, `SubStr`, `StrReplace`, `InStr`, `Trim`,
`LTrim`, `RTrim`, `StrCompare`, `StrSplit`, `Sort`, `RegExMatch`,
`RegExReplace`), objectos (`Type`, `IsObject`, `HasProp`, `HasMethod`,
`GetMethod`, `HasBase`, `IsSetRef`, `ObjGetBase`, `ObjSetBase`,
`ObjOwnProps`, `ObjOwnPropCount`, `ObjHasOwnProp`, `ObjGetCapacity`,
`ObjSetCapacity`, `ObjAddRef`), ficheiros (`FileAppend`, `FileDelete`,
`FileExist`, `FileRead`, `DirCreate`, `DirDelete`, `DirExist`), ecrã e rato
(`PixelGetColor`, `PixelSearch`, `Screenshot`, `Click`, `MouseMove`,
`MouseGetPos`), janelas (`WinActive`, `WinExist`, `WinGetTitle`), e o resto
(`MsgBox`, `Send`, `KeyWait`, `SendMode`, `SetKeyDelay`, `SetTimer`, `Sleep`,
`Run`, `RunWait`, `Exit`, `ExitApp`, `EnvGet`, `EnvSet`, `SplitPath`,
`VarSetStrCapacity`, `DllCall`).

As **131 variáveis** de v2 estão todas carryadas: as 40 que o runtime sabe
(`A_AhkVersion`, `A_Index`, `A_ScreenWidth`, `A_ScreenHeight`, `A_TickCount`,
`A_PtrSize`, `A_Space`, `A_Tab`, `A_EndChar`, `A_ThisFunc`, …) e as outras
com a razão.

### 3.3 O que a caixa faz no Linux, medido

| Primitiva | Mecanismo | Medição (2026-10-25 / 27 / 28) |
|---|---|---|
| Texto e teclas | teclado **uinput** próprio, layout US | `Send "Teste-Ahk 42!"` escreve a string inteira, maiúsculas e pontuação incluído |
| Modificadores | descem antes da tecla, sobem depois | `ctrl+a` selecciona o texto de um campo GTK e o backspace apaga |
| Ponteiro | **uinput** absoluto, `INPUT_PROP_DIRECT` | clique em `(830, 316)` atravessa o ponteiro |
| Ecrã | `grim` | `PixelGet 100, 100` → `#12101b` |
| Janelas | `xprop` (Xwayland do gamescope) | `WinExist`/`WinActive` → *"unknown"*, com a razão |
| Chamadas nativas | `dlopen`/`dlsym` + trampolim em `asm!` | 13 casos verdes no Windows, 14 verdes na caixa |
| DLLs do Windows | **raven** (PE de 18 KB, freestanding) | a caixa passa a chamar DLLs do Windows |
| Ficheiros | `std::fs` | `FileAppend("hello", "ahk_probe.txt")` escreve no directório de trabalho |

Limites de segurança: `MAX_STEPS` 20 000, `MAX_ITERATIONS` 1 000,
`MAX_DEPTH` 1 000, `MAX_STACK` 512 KB, `Sleep` ≤ 60 000 ms, timeout do runner
60 s. Relatório JSON: `ok`, `steps`, `output`, `notes`, `error`, `error_line`,
`exit_code`, `elapsed_ms`, `variables`.

### 3.4 O que ainda são esboços

Na árvore de trabalho de hoje:

| Ficheiro | Estado |
|---|---|
| `crates/ahk/src/hotkeys.rs` | esboço — `run_fired` devolve `Ok(())` |
| `crates/ahk-box/src/hotkeys.rs` | esboço |
| `crates/ahk/src/files.rs` | esboço (só o cabeçalho) |
| `crates/ahk-box/src/files.rs` | esboço |
| `crates/ahk/src/date.rs` | `DateTime` e `WallClock` definidos; `system_wall_clock()` devolve `None` |
| `crates/ahk/src/settings.rs` | esboço |

E as Classes: `Map`, `Array`, `Buffer` e `File` **funcionam**; `Gui`, `Menu` e
`Hotkey` são carryadas e **recusam** com a razão
(*"a menu is the menu bar of a Windows window, and this desktop is a Wayland
session without one"*).

---

## 4. Comparação

### 4.1 Cobertura por área

| Área | AHK_X11 | AHK (nosso) | Estado do nosso lado |
|---|---|---|---|
| **Hotkeys** | ✅ XGrabKey, opções v1, remap, BlockInput | ❌ recusa (variante 1) | **a maior lacuna** |
| **Hotstrings** | ✅ motor de palavra | ❌ recusa (variante 1) | depende dos hotkeys |
| **Threads** | ✅ cooperativas, interrupção, prioridade | ❌ um fio | grande mudança de modelo |
| **Envio de teclas** | ✅ XTEST, `Send`/`SendRaw` | ✅ **uinput** (melhor para Wayland) | ✅ à frente |
| **Rato** | ✅ XTEST | ✅ **uinput** absoluto | ✅ à frente |
| **Ecrã** | ✅ `XGetImage` | ✅ **`grim`** (Wayland) | ✅ à frente |
| **Janelas** | ✅ completo (xdotool fork) | ❌ *"unknown"* (Wayland sem lista) | precisa de backends |
| **Controlos (AT-SPI)** | ✅ texto, posição, clique | ❌ recusa (variante 0) | **oportunidade** |
| **GUI / diálogos** | ✅ GTK3 completo | ❌ recusa (variante 0) | precisa de toolkit |
| **Ficheiros** | ✅ 15 comandos | ⚠️ 12 dos 30 métodos do trait `Files` | trabalho mecânico |
| **INI** | ✅ | ❌ recusa (variante 4) | trabalho mecânico |
| **Som** | ✅ ffplay/vlc/paplay/amixer | ❌ recusa (variante 0) | **capacidade nova** |
| **Ambiente** | ✅ `EnvGet`/`EnvSet` | ✅ | ✅ igual |
| **Processos** | ✅ `Run`/`RunWait`/`RunAs`/`Shutdown` | ⚠️ `Run`/`RunWait` reais, o resto recusa | fácil |
| **`DllCall`** | ✅ FFI do Crystal (v1) | ✅ **dlopen + trampolim, v2, e o raven** | ✅ muito à frente |
| **Binário (`NumGet`/`NumPut`)** | ✅ | ❌ recusa (variante 0) | sobre a classe `Buffer` |
| **Clipboard** | ✅ GTK + back-off exponencial | ❌ recusa (variante 0) | **o mais barato de todos** |
| **Compilador para .exe** | ✅ | ❌ (o runner é o binário) | baixo valor para nós |
| **REPL / Window Spy / instalador** | ✅ | ❌ | opcional |

### 4.2 Os nomes que mudaram do v1 para o v2

Um script do AHK_X11 **não** corre no nosso tree, e quase sempre por causa de
um nome:

| v1 (AHK_X11) | v2 (nosso) | Nota |
|---|---|---|
| `MsgBox, texto` | `MsgBox texto` | vírgula já não é separador |
| `PixelGet` | `PixelGetColor` | |
| `Upper` / `Lower` | `StrUpper` / `StrLower` | |
| `StringLen` | `StrLen` | |
| `StringMid` / `StringLeft` / `StringRight` | `SubStr` | três num só |
| `StringTrimLeft` / `StringTrimRight` | `LTrim` / `RTrim` | |
| `StringSplit` | `StrSplit` | |
| `StringReplace` | `StrReplace` | assinatura diferente (a v2 tem o *case sense* e escreve o número de trocas) |
| `StringGetPos` | `InStr` | |
| `IfWinExist` | `WinExist` | |
| `IfWinActive` | `WinActive` | |
| `MouseClick` | `Click` | |
| `FileSelectFile` | `FileSelect` | |
| `FileSelectFolder` | `DirSelect` | |
| `VarSetCapacity` | `VarSetStrCapacity` | só o de texto |
| `IfMsgBox` | — | não existe em v2 |
| `Transform` | — | não existe em v2 |
| `RegExGetPos` | — | não existe em v2 |
| `EnvGet` | `EnvGet` | em v1 era preciso; em v2 também |
| `Hotstring` | `Hotstring` | comando, não directiva |
| `Gui, Add` | `Gui.Add` | classe, não comando |
| `Menu` | `Menu` | classe, não comando |

### 4.3 O que o AHK_X11 faz que nós recusamos — e porquê

Três recusas nossas que **não** devem ser revertidas:

1. **Janelas.** Recusamos porque a sessão da caixa é Wayland e o compositor
   não expõe lista de janelas. O AHK_X11 consegue porque o X11 expõe. Não é
   uma falha: é o sintoma correcto numa sessão Wayland.
2. **Injeção por XTEST.** O AHK_X11 injecta por XTEST porque vive em X11. Nós
   usamos uinput, que é o que funciona em Wayland — e que a caixa mediu.
3. **`DllCall` em estilo v1.** O AHK_X11 serializa tudo em texto porque as
   suas variáveis são texto. O nosso `DllCall` tem tipos, convenções, e
   escreve de volta — é a versão v2.

Uma recusa nossa que **deve** ser revista:

4. **Clipboard.** Recusamos por omissão do trait, não por uma limitação real
   da plataforma. `wl-paste`/`wl-copy` resolvem em Wayland, `xclip`/`xsel` em
   X11. Não há motivo técnico.

### 4.4 O que nós fazemos que o AHK_X11 não faz

- **O dialecto completo de v2.0.19**: expressões, classes, objectos, métodos,
  referência por `&`, `Prototype`/`Base`, o operador `is`, a família `Obj*`.
- **Nove tipos de classe registados** — que cobrem as onze tabelas do binário:
  `Map`, `Array`, `Buffer`, `File`, `Func`, `Gui`, `Menu`, `Hotkey`, `Object`.
- **Precedência e semântica medidas contra o binário** (`-2 ** 2` é -4;
  `"a" 1 + 2` é `"a3"`).
- **Núcleo `no_std`**: a linguagem compila para um alvo sem biblioteca padrão.
- **O relatório JSON**: cada passo, cada nota, a linha do erro, os limites.
- **Guard rails** em vez de threads: um script que se perde é parado com a
  linha do erro.
- **O `DllCall`** que fala com `.so` **e** com DLLs do Windows, no Linux, por
  um PE próprio.

### 4.5 As lições de método

1. **O ecrã é o trabalho; a linguagem é o que já está feito.** O AHK_X11 gastou anos em
   X11/GTK/AT-SPI e continua a ser um v1. Nós temos a linguagem certa e
   faltam as capacidades.
2. **Recusar com a razão é uma decisão de produto, não de engenharia.**
   O AHK_X11 recusa o que não faz (`EnvUpdate`, `RegRead`) removendo o
   comando. Nós respondemos com a razão — mais honesto, e é a regra do
   projecto (AGENTS.md: *"A question that cannot be answered fails, with the
   reason. It does not answer false."*).
3. **A acessibilidade é o caminho para as janelas em Wayland.** O AT-SPI é
   D-Bus, não X11. O AHK_X11 tem essa peça feita e medida.
4. **Hotkeys sem um servidor de janelas é metade do valor.** A AHK_X11 tem
   ambos; nós temos o input mas não o hook.

---

## 5. Sugestões: como dar suporte ao AHK_X11

### 5.0 A regra: portam-se as **capacidades**, não o mecanismo

O mecanismo do AHK_X11 (XTEST, XGrabKey, `XGetImage`, GTK3, `dlopen` do
Crystal) é o mecanismo do **X11**, e o nosso alvo é **Wayland**. Portar o
mecanismo seria um retrocesso.

O que se porta é o **contrato**:

- O que cada comando faz, com que argumentos e que erro dá.
- As **opções** que o comando aceita (`CoordMode`, `SendMode`, `SetKeyDelay`,
  `DetectHiddenWindows`, as opções de hotkey `t/c/p/i/w`).
- As **receitas** que os utilizadores escrevem (§5.4).

E a **costura** onde a capacidade assenta já existe: os traits de `Desktop` são o
fio. `Hotkeys`, `Clipboard` e `Windows` já têm o método e a assinatura
certas — falta o corpo.

### 5.1 Onda 1 — o que dá resultado já, com pouco risco

| # | Sugestão | Onde | Esforço | Impacto |
|---|---|---|---|---|
| 1 | **Clipboard real** — `wl-paste`/`wl-copy` (Wayland), `xclip`/`xsel` (X11) | `crates/ahk-box/src/desktop.rs`, `impl Clipboard` | **S** | `Clipboard`, `ClipWait` (com timeout), e a receita de "copiar, transformar, colar" |
| 2 | **Som** — `amixer` para o volume, `ffplay`/`paplay`/`aplay` para tocar, exactamente a escada do AHK_X11 | novo trait `Sound` no `ahk::desktop`, `impl Sound for LabDesktop` | **S** | `SoundGet`, `SoundSet`, `SoundPlay`, `SoundBeep` saem da recusa |
| 3 | **`NumGet` / `NumPut`** — sobre a classe `Buffer`, que já existe e funciona | `builtins.rs`, `classes/buffer.rs` | **S** | Sai da recusa de dois nomes que a v2 tem e o `Buffer` já dá soporte |
| 4 | **Acabar com os `Files`** — `copy_file`, `move_file`, `copy_dir`, `move_dir`, `file_attributes`, `set_file_attributes`, `file_times`, `set_file_times`, `ini_read`, `ini_write`, `ini_delete` | `crates/ahk-box/src/desktop.rs` + `crates/ahk/src/files.rs` | **M** | `FileCopy`, `FileMove`, `FileGetAttrib`, `FileSetAttrib`, `FileGetTime`, `FileSetTime`, `IniRead`, `IniWrite`, `IniDelete` saem da recusa. `std::fs` cobre quase tudo |
| 5 | **`FormatTime` / `DateAdd` / `DateDiff`** — `system_wall_clock()` deixou de devolver `None` | `crates/ahk/src/date.rs` | **M** | A data real, e `A_Year`, `A_Month`, `A_Day`, `A_Hour`, `A_Min`, `A_Sec`, `A_MSec`, `A_Now`, `A_NowUTC` |
| 6 | **`ahk --surface`** — um comando que imprima a tabela de 725 nomes com o estado de cada um (real / recusa / motivo) | `crates/ahk-cli/src/main.rs` | **S** | Torna visível a dívida; a medição de hoje é 93/253 e está numa linha de código |

O item 6 é o mais barato de todos e o mais útil: transforma "o que falta" de
opinião em número.

**O estado desta onda (2026-10-29).** Os seis itens estão feitos: o
clipboard (1), o som (2), o `NumGet` e o `NumPut` (3), os `Files` (4) e o
`FormatTime` com o `DateAdd` e o `DateDiff` (5) saíram da recusa no bloco da
secção 25 das notas, e o `ahk --surface` (6) imprime agora o estado de
cada um dos nomes de v2.

### 5.2 Onda 2 — hotkeys, threads, hotstrings

Esta é a onda que **muda o produto**: sem hotkeys, um AHK é um programa de
linha de comandos que mexe no rato.

**A costura já existe.** `ahk::desktop::Hotkeys` já tem
`register_hotkey`, `unregister_hotkey`, `hotkeys` e `fired_hotkey`, e o
documente do trait diz o que o runtime espera:

> *"the desktop watches the keyboard and says which of the hotkeys it
> registered was pressed; the runtime owns the function that the hotkey runs,
> and asks this between the statements of the script."*

O gancho está desenhado; **o corpo é que não existe**.

| # | Sugestão | Detalhe |
|---|---|---|
| 7 | **Backend de hotkeys para X11** | `XRecord` (ou `XGrabKey` como o AHK_X11) + `XTest`. Serve para X11 puro, e é o que permite testar sem Wayland |
| 8 | **Backend de hotkeys para Wayland** | Não existe grab global em Wayland. O caminho é o **uinput** + um watcher de `evdev` (o `ydotool`/`wtype` fazem isto), ou um portal. O custo é o mesmo que já pagámos no teclado: é um dispositivo |
| 9 | **Hotstrings** | `Hotstring` é um comando da v2 e está na recusa (variante 1). Depende dos hotkeys: é um hook que reescreve o texto quando a palavra bate |
| 10 | **Opções de hotkey** | `t` (transparent), `c` (context-sensitive), `p` (physical), `i` (ignore modifiers), `w` (wildcard), `x` (custom), `h` (hook). São o contrato do v1 e a v2 renomeou-as — mas o **conjunto** é o que os utilizadores conhecem |
| 11 | **`Hotkey` como classe** | A classe `Hotkey` de v2 tem 35 métodos (`Start`, `Stop`, `Wait`, `KeyOpt`, `OnKeyDown`, …). A AHK_X11 não tem esta noção; nós temos o esboço. Fazer a classe real é o caminho do v2 |
| 12 | **Threads: o modelo do AHK_X11 é a referência** | `src/run/thread.cr` é um guia concreto: interrupção cooperativa, prioridades, definições por thread. Mas o nosso runtime é "executa até ao fim" e o relatório JSON assume um fio |

Sobre o item 12, a recomendação é **não** correr atrás do modelo v1. O v2
também não tem threads: o v2 substituiu-as por **eventos e callbacks**
(`OnMessage`, `OnClipboardChange`, `OnExit`, timers). O caminho do v2 é:

1. Um **loop de eventos** no runtime: entre os statements, o runtime pergunta
   ao desktop por um hotkey disparado, um timer vencido, ou um evento.
2. O script continua a ser um script — a diferença é que o runtime pode
   *interromper* o statement corrente para despachar o evento.
3. O relatório JSON ganha um `thread`/`task` por passo, para que a
   interrogação continue a ser legível por máquina.

Isto é um trabalho de fundo, não uma tarefa. Vale a pena **depois** dos
hotkeys, e não antes.

### 5.3 Onda 3 — janelas, controlos e GUI

**A recusa de janelas continua correcta na caixa.** O compositor Wayland da
caixa não expõe lista de janelas ao D-Bus, e o Xwayland do gamescope reporta
uma lista de clientes vazia. A resposta certa continua a ser *"unknown"* com
a razão — não `false`.

O que a AHK_X11 ensina é que há **três caminhos**, e nós só temos o pior:

| Caminho | Mecanismo | Funciona em Wayland? | Esforço |
|---|---|---|---|
| **A. Backend X11** | `xprop` + `xdotool`/`x_do` | Não (mas funciona em X11) | **S** — a caixa já fala `xprop` |
| **B. Portal / compositor** | `xdg-desktop-portal`, D-Bus do COSMIC | Sim, se o compositor expõe | **M** — depende do compositor |
| **C. AT-SPI** | D-Bus, protocolo de acessibilidade | **Sim** | **M** — o AHK_X11 tem 20 KB de referência |

**O caminho C é o mais interessante.** O AT-SPI é a camada que a AHK_X11 usa
para `ControlGetText`, `ControlSetText`, `ControlGetPos`, `ControlClick` e
para o `WinText` do `WinGetText`. E é um protocolo **D-Bus**, que é
exactamente o que o Wayland preserva. Com ele, uma sessão Wayland ganha:

- `ControlGetText` / `ControlSetText` — ler e escrever o texto de um campo.
- `ControlGetPos` — a geometria de um controlo.
- `ControlClick` — clicar num controlo pelo nome, sem coordenadas.
- `WinGetText` — o texto de uma janela, que hoje recusamos.

O `xdotool` (caminho A) resolve o resto: activação, mover, dimensionar,
minimizar, fechar, esperar. É exactamente o `win-util.cr` do AHK_X11, e o
fork dele (`phil294/x_do.cr`) resolve o bug do `XGetWindowProperty` que faz
crashar o `xdotool` em alguns casos.

**Sugestão concreta:** um trait novo, `Accessibility`, com
`control_text`, `set_control_text`, `control_position`, `control_click` e
`window_text`, implementado por AT-SPI, com uma implementação por omissão
que recusa com a razão. Mais um trait na soma de `Desktop` — a regra do
projecto é que "uma capacidade nova vai no seu próprio trait, e o `Desktop`
cresce por a nomear".

**GUI (GTK).** O AHK_X11 faz `Gui`, `GuiControl`, `MsgBox`, `InputBox`,
`FileSelect`, `DirSelect`, `Menu`, `ToolTip` e `TrayTip` com GTK3. Para nós:

- `gtk4-rs` é a escolha (GTK4 é o que corre em Wayland sem XWayland).
- O `MsgBox` actual escreve no `output` do relatório JSON — **isso não deve
  mudar**: o runner tem de continuar a poder correr *headless* e a ser
  interrogável. A GUI tem de ser um **opt-in** (`--gui`), não o default.
- A classe `Gui` de v2 tem 59 nomes na superfície, e `Gui.Control` mais 29 —
  é uma API grande, e vale a pena um mapeamento directo de `gui-add.cr` do
  AHK_X11 para a nossa tabela de `Gui`.

### 5.4 Onda 4 — as receitas, que são o produto

O AHK_X11 traz **14 scripts de exemplo** em `docs/`:
`ContextSensitiveHelp`, `EasyWindowDrag`, `FavoriteFolders`, `JoystickMouse`,
`JoystickTest`, `KeyboardOnScreen`, `MinimizeToTrayMenu`, `MsgBoxButtonNames`,
`Numpad000`, `NumpadMouse`, `Seek`, `TooltipMouseMenu`, `VolumeOSD`,
`WindowShading`.

São scripts **v1**, e o nosso tree é **v2**: não correm. Mas são o melhor
corpus de requisitos que existe, porque cada um exercita uma combinação
real:

| Script | O que exige |
|---|---|
| `VolumeOSD` | som + `Progress` + `SetTimer` + `SoundGet`/`SoundSet` |
| `Seek` | `Gui` + `File` + `Loop Files` + `StringSplit` + `Run` |
| `KeyboardOnScreen` | hotkeys + desenho de teclas + `Input` |
| `NumpadMouse` | hotkeys + rato + `KeyWait` |
| `JoystickMouse` | leitura de dispositivo + rato |
| `MinimizeToTrayMenu` | `Menu` + `TrayTip` + eventos |
| `WindowShading` | `WinGet`/`WinMove`/`WinWait` + estado |
| `EasyWindowDrag` | rato + janelas em contínuo |
| `ContextSensitiveHelp` | `WinGetText` + `MouseGetPos` + `ToolTip` |
| `FavoriteFolders` | `IniRead`/`IniWrite` + `Run` + pastas |

**Sugestão:** portar cada um para v2 e usá-los como **casos de conformidade**
do nosso lado — em `crates/ahk/tests/language.rs` (o que é linguagem) e nos
scripts de verificação na caixa (o que é ecrã). O `AGENTS.md` já diz onde
cada caso deve morar; estes são casos de **ponta a ponta**, e por isso
pertencem à caixa, não ao crate da linguagem.

Isto também resolve o problema do "não sei o que falta": cada script é uma
lista de nomes que a árvore ainda recusa.

### 5.5 Onda 5 — o empacotamento

| # | Sugestão | Nota |
|---|---|---|
| 13 | **`ahk --compile script.ahk`** | Embutir o script no binário do runner e escrever um executável. É o que o AHK_X11 promete ("stand-alone binaries with no dependencies") e o que um utilizador final quer levar. Custo baixo: é um `include_bytes!` com um pequeno executor |
| 14 | **Window Spy** | O AHK_X11 tem o dele, em AHK. Nós temos as peças (leitura de pixel, posição do ponteiro) e a classe `Hotkey`. Um TTY simples que imprima a janela sob o rato e as teclas premidas |
| 15 | **REPL** | O AHK_X11 tem `--repl`. É útil para quem não está a desenvolver o runtime. Alternativa mais barata: um `--check` que faz *parse* e diz o que falha, sem executar |

### 5.6 O que **não** se porta

| Não portar | Porquê |
|---|---|
| A sintaxe legacy v1 | O nosso dialeto é v2.0.19, e é essa a vantagem competitiva. **Isto é verdade do v2, e só do v2**: o v1 passa a ser uma opção atrás de um gate, e a decisão vive em `docs/ahk-v1-dialect.md` |
| A convenção "output var" (`StringLen, out, in`) | Em v2 a resposta é o valor de retorno; a referência por `&` resolve o caso em que é preciso escrever |
| `#NoEnv` como default | Em v2 o ambiente é explícito; `EnvGet` está lá |
| O compilador Crystal e o *linker* | O item 13 cobre o essencial com muito menos |
| `x_do` (fork do xdotool) como dependência | Preferir o `xdotool` de sistema, ou implementar o X11 backend sobre `x11` + `xcb` em Rust |
| `AutoTrim` | A AHK_X11 pôs-o de parte pelo mesmo motivo: é ambíguo |
| O `PostMessage`/`SendMessage`/`Reg*` | São APIs do Windows. No v2 recusam com a razão, que é a resposta certa |

### 5.7 A nota da licença — **importante**

O AHK_X11 é **GPL-2.0**. Este projecto é **Apache 2.0**.

- **Pode**-se ler o código dele, estudá-lo, e implementar as *ideias*, as
  *receitas* e o *comportamento* de forma independente.
- **Não** se pode copiar o código-fonte dele para dentro deste repositório:
  a GPL-2.0 é copyleft e a Apache 2.0 é compatível com GPL-3.0, mas não com
  GPL-2.0. Uma cópia de código GPL-2.0 nesta árvore contaminaria a licença do
  projecto inteiro.

Isto não é um detalhe: `win-util.cr`, `at-spi.cr`, `send.cr` e `gui-add.cr`
são ~50 KB de código que **dá para ler e reescrever**, mas não para copiar.
Toda a sugestão acima foi escrita para ser implementada **de raiz**, a partir
da *descrição do comportamento* e do nosso próprio desenho.

---

## 6. Um plano por fases

| Fase | Conteúdo | Resultado verificável |
|---|---|---|
| **F0 — medir** | `ahk --surface`; a tabela de 725 nomes com o estado | 93/253 hoje; o número passa a ser visível em cada commit |
| **F1 — o que é barato** | Clipboard, Som, `NumGet`/`NumPut`, `Files` completo, `FormatTime`/`Date*` | ~25 nomes saem da recusa; o `Clipboard` e o `Sound` ganham traits próprios |
| **F2 — os hooks** | Backend de hotkeys (X11 e Wayland), `Hotstring`, a classe `Hotkey` real | `Hotkey`, `Hotstring`, `HotIf*` saem da recusa; um script passa a reagir a teclas |
| **F3 — o ecrã** | Backend X11 de janelas (`xdotool`), trait `Accessibility` por AT-SPI | `WinExist`/`WinActive`/`WinGetText`/`Control*` ganham resposta real |
| **F4 — as janelas** | `Gui`/`Menu`/`ToolTip`/`TrayTip` por GTK4, com `--gui` opt-in | a classe `Gui` deixa de recusar |
| **F5 — o corpo** | Loop de eventos, interrupção cooperativa, `OnExit`, `OnMessage` | o relatório JSON ganha a tarefa; os timers passam a ser reais |
| **F6 — as receitas** | Portar os 14 scripts de exemplo para v2 e usá-los como casos | a paridade passa a ser demonstrada, não afirmada |
| **F7 — o pacote** | `--compile`, Window Spy, `--check` | o entregável é um ficheiro |

As fases F0 e F1 são as que dão mais por hora. A F2 é a que muda o que o
projecto **é**. A F4 é a maior e a mais dependente do ambiente.

---

## 7. Como medir a paridade

O tree já tem a mecânica: `crates/ahk/tests/surface.rs` lê a tabela da
superfície e pergunta à árvore, nome a nome, se ela o carrega. A regra é
que a resposta pode ser um valor **ou a razão**, e um nome que a árvore não
carrega tem de estar na tabela de pendentes.

**O comando já existe.** `ahk --surface` imprime o estado de cada nome de v2,
sem script, e um resumo no fim:

```
name                 kind      state
Abs                  function  value
WinMove              command   refusal: this is a name of Windows, and this desktop is not Windows.
Hotkey               command   value
...
the surface of v2 has 725 entries: 479 answer with a value, 246 are refused, 0 are pending.
```

Com isso, "o que falta" deixa de ser uma opinião e passa a ser um número que
**não pode piorar sem se ver**. O JSON para o CI ainda não existe: é a mesma
informação, noutro formato, e o comando é o ponto de entrada.

Um segundo número merece atenção: **quantos nomes da superfície de v2 estão
implementados a sério** — hoje 479 de 725 entradas. A meta não é 725: v2 tem
nomes que são Windows e não vão acontecer aqui. A meta honesta é: *nenhum
nome que o AHK_X11 faz no Linux devia estar na recusa por falta de
implementação*. A AHK_X11 faz ~127 comandos de v1 num ambiente comparável;
muitos deles têm equivalente directo em v2. Essa é a régua de paridade que
propomos.

---

## 8. As fontes

- **AHK_X11** — documentação: <https://phil294.github.io/AHK_X11/>;
  código: <https://github.com/phil294/AHK_X11> (branch `master`,
  `shard.yml` versão 1.0.7). Lidos: `README.md`, `docs/list of commands.txt`,
  `src/ahk_x11.cr`, `src/run/display/{x11,gtk,at-spi,hotkey,hotstring,display-adapter}.cr`,
  `src/run/thread.cr`, `src/build/{parser,linker}.cr`,
  `src/cmd/**`, `src/util/ahk-bin-var.cr`.
- **AHK (este repositório)** — `AGENTS.md` (as notas de trabalho, §12–§23),
  `crates/ahk/src/{builtins,desktop,lib}.rs`,
  `crates/ahk/src/classes/*.rs`, `crates/ahk/src/{hotkeys,files,date,settings}.rs`,
  `crates/ahk/src/variables.rs`, `crates/ahk/reference/ahk-v2.0.19.surface.tsv`,
  `crates/ahk-box/src/{desktop,keyboard,pointer,native,raven}.rs`,
  `crates/ahk-cli/src/main.rs`.
- **Medições da caixa** — 2026-10-25, 2026-10-27 e 2026-10-28, em
  192.168.0.40 (CachyOS, Wayland, COSMIC). O registo completo está no
  `AGENTS.md` (§13).
- **Contagens desta comparação** — 725 entradas de superfície (644 nomes
  distintos): 242 comandos, 103 funções, 131 variáveis, 249 métodos; 346
  built-ins na tabela, 141 com handler real e 205 com `not_carried`. Das 725
  entradas, 479 respondem com um valor e 246 recusam com a razão — o
  `ahk --surface` é quem conta.
