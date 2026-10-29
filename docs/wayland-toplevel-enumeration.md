# Enumerar janelas num compositor Wayland

> Guia técnico e exemplo funcional em Rust, com `ext-foreign-toplevel-list-v1`.
>
> O código deste guia foi **compilado e executado** na caixa do laboratório
> (192.168.0.40, CachyOS, Wayland, COSMIC) a 2026-10-28. A saída real está
> na secção 7.

---

## 1. O modelo: porquê é que o Wayland não tem uma lista de janelas

O Wayland foi desenhado ao contrário do X11. No X11, qualquer cliente pode
perguntar ao servidor pela árvore de janelas inteira e receives os
identificadores de todas elas — é um problema de segurança conhecido, e é a
razão pela qual o `xprop`/`xdotool`/`wmctrl` funcionam em qualquer sessão X11
sem permissão nenhuma.

No Wayland, um cliente só vê **as superfícies que ele mesmo criou**. As
janelas dos outros programas são de outro cliente, e o compositor não as
annuncia a ninguém. Não existe `wl_display.get_windows()` e nunca vai
existir: o objectivo do protocolo é que um cliente malicioso não possa
descobrir o que está no ecrã do utilizador nem interferir com as janelas dos
outros.

A consequência prática é que **enumerar janelas é uma permissão, não um
direito**. O compositor concede essa permissão por um protocolo de
extensão, e o cliente tem de o pedir explicitamente — e o compositor pode
recusá-lo.

### 1.1 Os dois protocolos que interessam

| Protocolo | Lista | Controla | Quem publica |
|---|---|---|---|
| `ext-foreign-toplevel-list-v1` | sim | **não** | compositor que implemente a listagem |
| `zwlr-foreign-toplevel-management-unstable-v1` | sim | **sim** | wlroots (Sway, Hyprland, Wayfire, riverlab, ...) |
| `zcosmic_toplevel_manager_v1` | sim | **sim** | COSMIC |

A diferença é o ponto inteiro:

- **`ext-foreign-toplevel-list-v1`** é o protocolo **mínimo e portátil**. O
  compositor entrega uma *handle* por cada toplevel mapeado, e a handle diz
  o que a janela é: `title`, `app_id` e um `identifier` estável. **Não há
  nenhum request de acção** — o cliente não pode activar, mover, minimizar
  nem fechar nada. É um protocolo de *observação*.
- **`zwlr-foreign-toplevel-management-unstable-v1`** (e o equivalente da
  COSMIC) acrescenta os requests de acção: `activate`, `minimize`,
  `unmaximize`, `maximize`, `close`, `move`, `resize`, além dos eventos de
  estado (`activated`, `fullscreen`, `minimized`) e de output. É um
  protocolo de *controlo*.

O `ext-` é a versão **staging** do protocolo, levada para o
wayland-protocols para substituir o `zwlr-` unstable. É o mesmo desenho, sem o poder de agir.
A razão de o `ext-` existir é a mesma que fez o X11 ser considerado
inseguro: a lista é útil, o controlo é perigoso, e um compositor pode
querer dar a primeira e negar o segundo.

> **Medido na caixa (2026-10-28).** O compositor é o `cosmic-comp`. Publica
> `ext_foreign_toplevel_list_v1` (versão 1), `zcosmic_toplevel_manager_v1`,
> `zcosmic_toplevel_info_v1`, `ext_data_control_manager_v1`,
> `ext_workspace_manager_v1`, `ext_foreign_toplevel_image_capture_source_manager_v1`,
> `cosmic_a11y_manager_v1` e `wp_pointer_warp_v1`. **Não** publica
> `zwlr_foreign_toplevel_management_v1` — o COSMIC não é wlroots — mas
> publica o seu equivalente, com os mesmos poderes.

### 1.2 O que isto muda para o nosso runtime

O `AGENTS.md` dizia que `WinExist` e `WinActive` respondiam *"unknown"* na
caixa, porque o compositor Wayland não expõe lista de janelas ao D-Bus e o
Xwayland do gamescope reporta uma lista vazia. **Isso continua verdadeiro
para o D-Bus, e deixa de ser verdade para o Wayland**: o `cosmic-comp`
publica `ext_foreign_toplevel_list_v1`, e o programa deste guia enumera as
janelas da caixa sem dificuldade nenhuma. O que faltava era o caminho
Wayland-native, não a permissão.

**Esse caminho está feito (2026-10-29).** O cliente vive em
`crates/ahk-box/src/wayland.rs`, e `WinGetTitle`, `WinExist` e `WinActive`
respondem com valor na caixa. Duas coisas que este guia não cobre e que o
cliente teve de aprender:

- **A janela activa vem de um segundo protocolo.** O `ext-foreign-toplevel-list-v1`
  não diz qual das janelas tem o foco. Quem diz é o
  `zcosmic-toplevel-info-v1`, que *estende* a listagem: `get_cosmic_toplevel`
  recebe a handle da listagem e devolve uma handle do COSMIC, e essa é que
  carrega o `state`. A correlação é pela handle, portanto é exacta — e não
  um palpite pelo título.
- **O `state` chega um frame depois.** O compositor diz o estado da janela
  no seu próximo refresco, e um refresco é um frame: medido na caixa, a
  resposta estava na segunda pergunta, dez milissegundos depois da
  primeira. O cliente espera — com limite — antes de dizer que a janela
  activa é uma ou outra.

O XML dos dois protocolos está vendorizado em
`crates/ahk-box/protocols/` (MIT), e as ligações são geradas pelo
`wayland-scanner` em tempo de compilação. O crate `cosmic-protocols` é
GPL-3.0 e **não** é dependência desta árvore: o comportamento é portado, o
código não é copiado.

---

## 2. `Cargo.toml`

```toml
[package]
name = "wl-toplevels"
version = "0.1.0"
edition = "2021"
rust-version = "1.71"
description = "Enumerate Wayland toplevels with ext-foreign-toplevel-list-v1"
license = "MIT OR Apache-2.0"

[dependencies]
anyhow = "1"
wayland-client = "0.31"
wayland-protocols = { version = "0.32", features = ["client", "staging"] }
wayland-backend = "0.3"
rustix = { version = "1", features = ["event"] }
signal-hook = "0.3"

[profile.release]
opt-level = "s"
lto = true
```

Três notas sobre o manifesto:

1. **`features = ["client", "staging"]` é obrigatório.** O
   `ext-foreign-toplevel-list-v1` é um protocolo *staging*, e o crate
   `wayland-protocols` esconde os protocolos staging atrás dessa feature.
   Sem ela, `wayland_protocols::ext::foreign_toplevel_list` nem existe, e o
   erro é um `unresolved import` que não diz nada sobre a causa.
2. **`wayland-scanner` não é uma dependência.** O `wayland-protocols` já
   traz o código gerado; o `wayland-scanner` só seria preciso para um
   protocolo que não esteja no crate (ver a secção 8).
3. **`rustix`** entra pelo `event`: é o `poll(2)` que o `wayland-client` usa
   por dentro, e é ele que permite esperar pelo compositor com um tempo
   limite em vez de bloquear para sempre.

---

## 3. `src/main.rs`

```rust
//! Enumerate the toplevel windows of a Wayland compositor.
//!
//! # The model of security
//!
//! The core of Wayland has no list of windows. A client sees only the
//! surfaces it created itself; every other window belongs to another
//! client, and the compositor does not tell anybody about them. There is
//! no `wl_display.get_windows()`, and there never will be one: a window
//! is not the business of the client that watches the screen.
//!
//! The enumeration is therefore a privilege, and the compositor grants it
//! through an extension protocol. Two of them matter:
//!
//! * `ext-foreign-toplevel-list-v1` -- lists the toplevels and says what
//!   they are (title, application id, stable identifier). It cannot do
//!   anything to them.
//! * `zwlr-foreign-toplevel-management-unstable-v1` -- lists them *and*
//!   hands the client the power to act on them: activate, minimize,
//!   maximize, close, move, resize.
//!
//! The compositor of the lab box is COSMIC, and it publishes its own
//! equivalent of the second one, `zcosmic_toplevel_manager_v1`, with the
//! same powers.
//!
//! This program uses the first one, because the question it answers is
//! "which windows are there".
//!
//! # The events
//!
//! Binding the global starts the snapshot: the compositor sends one
//! `toplevel` event for every mapped toplevel, and for each of them a
//! handle that carries the properties. The protocol says that a roundtrip
//! after the bind is what tells the client that the snapshot is complete:
//!
//! > Clients which only care about the current state can perform a
//! > roundtrip after binding this global.
//!
//! After that, the same events carry the changes: a `toplevel` event is a
//! window that was mapped, a `closed` event is a window that went away.
//!
//! The properties of a handle arrive one by one and are only to be applied
//! when the `done` event confirms them, so that a title and an application
//! id that change together are seen together.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use signal_hook::consts::SIGINT;
use signal_hook::flag;
use wayland_backend::client::WaylandError;
use wayland_client::globals::{registry_queue_init, BindError, GlobalListContents};
use wayland_client::protocol::wl_registry::{self, WlRegistry};
use wayland_client::{event_created_child, Connection, Dispatch, EventQueue, Proxy, QueueHandle};
use wayland_protocols::ext::foreign_toplevel_list::v1::client::{
    ext_foreign_toplevel_handle_v1::{Event as HandleEvent, ExtForeignToplevelHandleV1},
    ext_foreign_toplevel_list_v1::{self, Event as ListEvent, ExtForeignToplevelListV1},
};

/// How long the loop waits for the compositor before it looks at the
/// Ctrl+C flag again. Short enough to feel immediate, long enough not to
/// spin on an idle desktop.
const POLL_TIMEOUT: Duration = Duration::from_millis(100);

/// How long the program waits for the `finished` that answers the `stop`
/// request at the end.
const STOP_TIMEOUT: Duration = Duration::from_millis(500);

fn main() -> Result<()> {
    // Ctrl+C does not kill the program: it sets the flag, and the loop
    // looks at it between two waits. The alternative -- a signal handler
    // that exits -- would leave the compositor with half a protocol.
    let interrupted = Arc::new(AtomicBool::new(false));
    flag::register(SIGINT, Arc::clone(&interrupted)).context("cannot install the Ctrl+C handler")?;

    let conn = Connection::connect_to_env().context("cannot connect to the Wayland display")?;

    // The registry is read here, and the queue that will carry the events
    // is created with it.
    let (globals, mut queue) =
        registry_queue_init::<App>(&conn).context("cannot read the registry of the compositor")?;

    let mut app = App::default();

    // The global is the door to the list of the windows. A compositor that
    // does not publish it does not let anybody list them, and there is no
    // other way in.
    match globals.bind::<ExtForeignToplevelListV1, _, _>(&queue.handle(), 1..=1, ()) {
        Ok(list) => app.list = Some(list),
        Err(BindError::NotPresent) => {
            eprintln!(
                "the compositor does not publish ext_foreign_toplevel_list_v1:\n\
                 \x20 the windows cannot be listed through Wayland.\n\
                 \x20 `wayland-info` says which globals it does publish."
            );
            // The program keeps running: a compositor is allowed to
            // publish the global later, and the registry handler below
            // picks it up when it does.
        }
        Err(e) => return Err(e).context("cannot bind ext_foreign_toplevel_list_v1"),
    }

    // The snapshot: one `toplevel` event for every mapped window, and the
    // properties of each of them. The roundtrip is what says when it is
    // over.
    queue.roundtrip(&mut app).context("the roundtrip failed")?;
    app.snapshot_done = true;
    app.print_snapshot();

    // The loop: wait for the compositor, dispatch what it sent, and look
    // at the Ctrl+C flag in between.
    while !interrupted.load(Ordering::Relaxed) {
        wait_for_events(&queue, &conn, POLL_TIMEOUT)?;
        queue.dispatch_pending(&mut app).context("cannot dispatch the events")?;
    }

    println!("\nCtrl+C: stopping.");

    // The polite way out: ask the compositor to stop sending events, wait
    // for the `finished` that answers the request, and then let the
    // objects go. The connection is closed right after, so the wait is
    // bounded.
    if app.list.is_some() {
        app.list.as_ref().expect("the list is there").stop();
        conn.flush().context("cannot flush the connection")?;

        let deadline = Instant::now() + STOP_TIMEOUT;
        while !app.finished && Instant::now() < deadline {
            wait_for_events(&queue, &conn, Duration::from_millis(50))?;
            queue.dispatch_pending(&mut app).context("cannot dispatch the events")?;
        }
    }

    Ok(())
}

/// Wait for the compositor to say something, but not for longer than the
/// timeout.
///
/// The read is prepared *before* the wait: the guard is what tells the
/// other threads of the queue that somebody is reading, and dropping it
/// cancels the prepared read. This is what keeps the event queue from
/// being blocked: the program is never inside a read that it cannot leave.
fn wait_for_events(
    queue: &EventQueue<App>,
    conn: &Connection,
    timeout: Duration,
) -> Result<()> {
    let Some(guard) = queue.prepare_read() else {
        // Another thread holds the read; there is nothing to wait for.
        return Ok(());
    };

    // Send what we have before waiting, or the compositor never hears it.
    conn.flush().context("cannot flush the connection")?;

    let fd = guard.connection_fd();
    let mut fds = [rustix::event::PollFd::new(&fd, rustix::event::PollFlags::IN)];
    let deadline = rustix::event::Timespec {
        tv_sec: timeout.as_secs() as i64,
        tv_nsec: timeout.subsec_nanos() as i64,
    };

    let ready = match rustix::event::poll(&mut fds, Some(&deadline)) {
        Ok(ready) => ready,
        // A signal arrived while we waited -- the Ctrl+C, most likely.
        // The loop looks at the flag and decides what to do about it; the
        // wait itself was not a failure.
        Err(e) if e.kind() == std::io::ErrorKind::Interrupted => 0,
        Err(e) => return Err(e).context("cannot wait for the compositor"),
    };

    if ready > 0 {
        // The socket has something. Reading it puts the events in the
        // queue; the handlers run in `dispatch_pending`.
        if let Err(e) = guard.read() {
            // Another thread took the events between the poll and the
            // read. That is not an error.
            let would_block = matches!(&e, WaylandError::Io(io) if io.kind() == std::io::ErrorKind::WouldBlock);
            if !would_block {
                return Err(e).context("cannot read the events of the compositor");
            }
        }
    }

    Ok(())
}

/// What the program knows about one toplevel.
#[derive(Debug, Default)]
struct Toplevel {
    /// The stable identifier of the toplevel, when the compositor sends
    /// one. It is the same string for the same window across processes, so
    /// it is the one to show to a human and to write down.
    identifier: Option<String>,
    /// The title, as the compositor last confirmed it.
    title: String,
    /// The application id, as the compositor last confirmed it.
    app_id: String,
    /// Whether this is a toplevel whose properties are still coming.
    fresh: bool,
    /// The title that arrived and waits for the `done` that confirms it.
    pending_title: Option<String>,
    /// The application id that arrived and waits for its `done`.
    pending_app_id: Option<String>,
    /// The identifier that arrived; the compositor sends it only once.
    pending_identifier: Option<String>,
}

impl Toplevel {
    /// The line that describes this toplevel.
    fn line(&self, id: u32) -> String {
        format!(
            "[0x{id:08x}] identifier={} title={:?} app_id={:?}",
            self.identifier.as_deref().unwrap_or("-"),
            self.title,
            self.app_id,
        )
    }
}

/// The state of the program.
#[derive(Default)]
struct App {
    /// The toplevels, by the numeric id of the handle that represents
    /// them. The map is ordered, so that the snapshot is printed in a
    /// stable order.
    toplevels: BTreeMap<u32, Toplevel>,
    /// The handles, kept alive on purpose: dropping a proxy destroys the
    /// object on the server, and a destroyed handle is a handle that will
    /// never send anything else.
    handles: Vec<ExtForeignToplevelHandleV1>,
    /// The list object, kept alive for the same reason.
    list: Option<ExtForeignToplevelListV1>,
    /// Whether the initial snapshot is over.
    snapshot_done: bool,
    /// Whether the compositor said it is done sending events.
    finished: bool,
}

impl App {
    /// Print the snapshot of the windows that are there.
    fn print_snapshot(&self) {
        println!("--- snapshot: {} mapped toplevel(s) ---", self.toplevels.len());
        for (id, toplevel) in &self.toplevels {
            println!("{}", toplevel.line(*id));
        }
        println!("--- the snapshot is over; the changes follow ---");
    }
}

/// The registry: the list of the globals of the compositor.
impl Dispatch<WlRegistry, GlobalListContents> for App {
    fn event(
        state: &mut Self,
        proxy: &WlRegistry,
        event: wl_registry::Event,
        _data: &GlobalListContents,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        else {
            return;
        };

        if interface != ExtForeignToplevelListV1::interface().name {
            return;
        }

        // A compositor may publish the global after the program started.
        // The version of the protocol is 1, and the compositor may
        // announce a higher one; the program asks for the one it knows.
        let list: ExtForeignToplevelListV1 = proxy.bind(name, version.min(1), qh, ());
        state.list = Some(list);
        println!("the compositor published {interface} (version {version}); it is bound now.");
    }
}

/// The list: the windows that are there, and the ones that come.
impl Dispatch<ExtForeignToplevelListV1, ()> for App {
    fn event(
        state: &mut Self,
        _proxy: &ExtForeignToplevelListV1,
        event: ListEvent,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            ListEvent::Toplevel { toplevel } => {
                let id = toplevel.id().protocol_id();
                let entry = state.toplevels.entry(id).or_default();
                entry.fresh = true;
                // The handle has to live as long as the program does.
                state.handles.push(toplevel);
            }
            ListEvent::Finished => {
                // The compositor answered the `stop` request; it will not
                // send anything else on this object.
                state.finished = true;
            }
            // The generated enum is `#[non_exhaustive]`: an event that a
            // later version of the protocol adds lands here.
            _ => {}
        }
    }

    // The `toplevel` event creates the handle object, and a child object
    // needs its user data before it can be used. The macro is what says
    // which event creates which object, and what data it carries.
    event_created_child!(App, ExtForeignToplevelListV1, [
        ext_foreign_toplevel_list_v1::EVT_TOPLEVEL_OPCODE => (ExtForeignToplevelHandleV1, ()),
    ]);
}

/// The handle: the properties of one window.
impl Dispatch<ExtForeignToplevelHandleV1, ()> for App {
    fn event(
        state: &mut Self,
        proxy: &ExtForeignToplevelHandleV1,
        event: HandleEvent,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        let id = proxy.id().protocol_id();
        // The snapshot flag is read before the borrow of the map, or the
        // two borrows would fight.
        let snapshot_done = state.snapshot_done;

        let Some(toplevel) = state.toplevels.get_mut(&id) else {
            // A handle of a window that was closed before we got here.
            return;
        };

        let mut closed = false;

        match event {
            HandleEvent::Identifier { identifier } => {
                toplevel.pending_identifier = Some(identifier);
            }
            HandleEvent::Title { title } => {
                toplevel.pending_title = Some(title);
            }
            HandleEvent::AppId { app_id } => {
                toplevel.pending_app_id = Some(app_id);
            }
            HandleEvent::Done => {
                // The properties are confirmed: now they are the truth.
                let mut changed = false;
                if let Some(identifier) = toplevel.pending_identifier.take() {
                    toplevel.identifier = Some(identifier);
                    changed = true;
                }
                if let Some(title) = toplevel.pending_title.take() {
                    toplevel.title = title;
                    changed = true;
                }
                if let Some(app_id) = toplevel.pending_app_id.take() {
                    toplevel.app_id = app_id;
                    changed = true;
                }
                if !changed {
                    return;
                }

                let fresh = toplevel.fresh;
                toplevel.fresh = false;
                if snapshot_done {
                    // A window that was mapped after the snapshot is a
                    // new one; a window that was already there only
                    // changed.
                    let mark = if fresh { "+" } else { "~" };
                    println!("{mark} {}", toplevel.line(id));
                }
            }
            HandleEvent::Closed => {
                // The window is gone, and the compositor will not send
                // anything else on this handle. The protocol asks the
                // client to destroy the handle, and the window leaves
                // the table.
                println!("- {} closed", toplevel.line(id));
                if let Some(pos) = state
                    .handles
                    .iter()
                    .position(|handle| handle.id().protocol_id() == id)
                {
                    let handle = state.handles.remove(pos);
                    handle.destroy();
                }
                closed = true;
            }
            // The generated enum is `#[non_exhaustive]`: an event that a
            // later version of the protocol adds lands here.
            _ => {}
        }

        if closed {
            state.toplevels.remove(&id);
        }
    }
}
```

---

## 4. Compilar e executar

O crate é **Linux-only**: o `wayland-sys` usa `std::os::unix`, e por isso
não compila no Windows. Na caixa:

```sh
# 1. o código vai para a caixa primeiro: um build sem a cópia compila
#    o que a caixa ainda tem, e o resultado é um binário velho sem erro.
ssh danalec@192.168.0.40 'mkdir -p ~/src/wl-toplevels/src'
scp Cargo.toml danalec@192.168.0.40:~/src/wl-toplevels/Cargo.toml
scp src/main.rs danalec@192.168.0.40:~/src/wl-toplevels/src/main.rs

# 2. o build
ssh danalec@192.168.0.40 'cd ~/src/wl-toplevels && cargo build --release'

# 3. a execução, na sessão do utilizador
ssh danalec@192.168.0.40 'cd ~/src/wl-toplevels && \
    XDG_RUNTIME_DIR=/run/user/1000 WAYLAND_DISPLAY=wayland-1 \
    ./target/release/wl-toplevels'
```

Numa sessão gráfica normal (terminal dentro do Wayland), basta:

```sh
cargo run --release
```

O `Connection::connect_to_env()` lê `WAYLAND_DISPLAY` e `XDG_RUNTIME_DIR`.
Sem `WAYLAND_DISPLAY`, ele falha com *"cannot connect to the Wayland
display"* — e isso é o sintoma de um `ssh` sem o ambiente da sessão, não um
erro do programa.

**Ctrl+C** põe a flag, o loop vê-a à próxima volta, e o programa sai pelo
caminho limpo: pede `stop`, espera o `finished`, e fecha a ligação. O código
de saída é 0.

---

## 5. Os eventos: o que é snapshot e o que é incremental

Esta é a parte que engana, e a fonte de quase todos os erros neste
protocolo.

### 5.1 O snapshot inicial

```
bind no global
   │
   ├── event  ext_foreign_toplevel_list_v1.toplevel   (handle #1)
   │     ├── event  handle.identifier = "P6jNlxgK..."     ┐
   │     ├── event  handle.title      = "Firefox"        ├─ as propriedades
   │     ├── event  handle.app_id     = "firefox"        ┘
   │     └── event  handle.done                          ← confirma
   │
   ├── event  ext_foreign_toplevel_list_v1.toplevel   (handle #2)
   │     └── ... idem ...
   │
   └── roundtrip (wl_display.sync)                        ← o snapshot acabou
```

O compositor envia **um `toplevel` por cada toplevel mapeado**, e logo a
seguir as propriedades de cada handle (`identifier`, `title`, `app_id`),
seguidas de `done`.

**O fim do snapshot é marcado pelo roundtrip, não por um evento.** O
protocolo diz exactamente isso:

> Clients which only care about the current state can perform a roundtrip
> after binding this global.

Um `wl_display.sync` que o compositor responde depois de ter processado tudo
o que havia antes garante que os eventos do snapshot já chegaram. Por isso
o programa faz `queue.roundtrip(&mut app)?` logo depois do bind, e só
depois imprime a tabela.

> **Erro comum.** O evento `finished` **não** marca o fim do snapshot. O
> `finished` é a resposta ao request `stop`, e o compositor só o envia
> depois de o cliente pedir para parar de receber eventos. Confundir os dois
> dá um programa que nunca imprime a tabela inicial.

### 5.2 As propriedades aplicam-se no `done`

O protocolo é explícito: *"The configured state must not be applied
immediately"* — o `title` e o `app_id` que chegam **não** se aplicam no
momento em que chegam, mas no `done` que os confirma. Assim, um título e um
`app_id` que mudam juntos são vistos juntos, e o programa nunca mostra um
estado intermédio que o utilizador nunca viu.

É por isso que o código tem os campos `pending_title`, `pending_app_id` e
`pending_identifier`: o que chegou espera, e o `done` move o pendente para
o aplicado.

### 5.3 As Actualizações incrementais

Depois do roundtrip, os mesmos eventos passam a querer dizer outra coisa:

| Evento | Antes do roundtrip | Depois do roundtrip |
|---|---|---|
| `toplevel` | uma janela do inventário | **uma janela nova** foi mapeada |
| `identifier`/`title`/`app_id` | as propriedades iniciais | uma propriedade **mudou** |
| `done` | confirma o inicial | confirma a mudança |
| `closed` | — | **a janela foi fechada** |

O programa distingue os dois regimes com a flag `snapshot_done`: durante o
snapshot, nada é impresso (a tabela é impressa de uma vez, no fim); depois,
cada `done` imprime a linha com `+` (janela nova) ou `~` (janela que mudou),
e o `closed` imprime `-`.

O `identifier` merece uma nota: é **estável** e o compositor só o envia
quando a handle é criada. Ele serve para dizer se duas handles são a mesma
janela, e é o mesmo valor em processos diferentes — é o `id` que se escreve
num log e se usa mais tarde.

### 5.4 `closed` e a destruição da handle

Depois do `closed`, o compositor não envia mais nada naquela handle, e o
protocolo pede ao cliente que a destrua. O programa faz as duas coisas:
imprime a linha e chama `destroy()` na handle, tirando-a da lista.

---

## 6. Como não bloquear a event queue

`EventQueue::blocking_dispatch()` é o caminho fácil, e é o errado para um
programa que precisa de responder a Ctrl+C: ele bloqueia dentro do
`read(2)` do socket, e o `SIGINT` chega ao processo, mas o programa não
volta a correr para o ver.

O padrão certo é o **prepare-read** (é o que a própria documentação do
crate descreve):

1. `queue.prepare_read()` — devolve um guard. Este passo **não** bloqueia.
   O guard é o que coordena com as outras threads da queue: só uma thread
   pode ler do socket, e o guard garante isso.
2. `conn.flush()` — envia o que está pendente. Sem isto, um request feito
   antes da espera nunca chega ao compositor.
3. `poll(2)` no `guard.connection_fd()`, **com um tempo limite**. O
   `poll` com timeout é o que dá ao programa a chance de olhar para a flag
   do Ctrl+C mesmo quando o compositor está calado.
4. `guard.read()` — só quando o `poll` disse que há dados. A leitura põe os
   eventos na queue; os handlers correm em `dispatch_pending()`.
5. Se o `poll` expirou, o guard é largado, e o `prepare_read` é cancelado.

O resultado é que **o programa nunca está dentro de uma leitura da qual
não possa sair**, e o loop é o mesmo para o `poll` que expira e para o que
devolve eventos.

Dois erros que só aparecem em execução:

- **`EINTR`.** O `SIGINT` interrompe o `poll(2)` e ele devolve
  `ErrorKind::Interrupted`. Se isso não for tratado, o programa morre com
  *"Interrupted system call"* em vez de sair limpo. O código trata-o como
  "não aconteceu nada", e o loop olha para a flag à volta seguinte.
- **`WouldBlock` no `read()`.** Se outra thread da queue leu os eventos
  entre o `poll` e o `read`, o `read` devolve `WouldBlock`. Também não é um
  erro.

E uma armadilha do protocolo, não do Rust: **um proxy Wayland que é
largado destrói o objecto no servidor.** Por isso o programa guarda as
handles em `state.handles` e o objecto da lista em `state.list`: sem isso,
o handle é destruído no instante em que chega, e a janela desaparece do
inventário antes de mandar o `title`.

---

## 7. Saída esperada (medida na caixa)

Medido a 2026-10-28, na caixa do laboratório (COSMIC, sessão Wayland), com
uma janela do Chromium e dois terminais abertos:

```
--- snapshot: 3 mapped toplevel(s) ---
[0xff000000] identifier=P6jNlxgKyYzywzlT0DEhIpIkDjEMuTq8 title="pod153/Login - Chromium" app_id="chromium"
[0xff000001] identifier=KQhaYikN2lfsB10mfujYMPK32Ob48Dsi title="agy --dangerously-skip-permissions" app_id="kitty"
[0xff000002] identifier=BmWetEuIFvUaMMZj4VWUow1Cp0pdriKd title="danalec@pod158c:~" app_id="kitty"
--- the snapshot is over; the changes follow ---
```

E, com uma janela que abre e fecha enquanto o programa corre:

```
+ [0xff000003] identifier=G5OripXB5z4rDaCWM0ovuxIEpHuzxRj5 title="wl-probe-2" app_id="kitty"
- [0xff000003] identifier=G5OripXB5z4rDaCWM0ovuxIEpHuzxRj5 title="wl-probe-2" app_id="kitty" closed

Ctrl+C: stopping.
```

O `identifier` tem 32 caracteres ASCII imprimíveis, como o protocolo
promete. O `0xff000000` é o id do objecto Wayland da handle, e só serve
para distinguir handles dentro da mesma sessão.

---

## 8. Limitações, e como testar o support

### 8.1 Se o compositor não publica o global

Não há enumeração possível por Wayland nativo. O programa diz isso na
saída de erro:

```
the compositor does not publish ext_foreign_toplevel_list_v1:
 the windows cannot be listed through Wayland.
 `wayland-info` says which globals it does publish.
```

E continua a correr, porque um compositor pode publicar o global mais
tarde; o handler do registry apanha-o quando isso acontecer.

### 8.2 Como detectar o support

Com o `wayland-info`, que é o modo canónico -- o formato da saída é este:

```
ext_foreign_toplevel_list_v1 (v1):
    interface: 'ext_foreign_toplevel_list_v1'
    version: 1
```

Sem o `wayland-info` instalado -- e na caixa ele **não** está instalado --
o `WAYLAND_DEBUG=1` de qualquer cliente Wayland imprime o registry inteiro:

```sh
WAYLAND_DEBUG=1 <qualquer-cliente-wayland> 2>&1 | grep 'wl_registry@.*global'
```

E o que a caixa responde (2026-10-28), em ordem:

```
wl_compositor, wl_subcompositor, wl_data_device_manager, wl_shm, wl_output,
wl_seat, xdg_wm_base, xdg_activation_v1, wl_drm, wl_fixes,
ext_foreign_toplevel_list_v1,          <-- a listagem
ext_data_control_manager_v1,           <-- a clipboard
ext_workspace_manager_v1,              <-- as workspaces
ext_foreign_toplevel_image_capture_source_manager_v1,   <-- a captura de uma janela
ext_image_copy_capture_manager_v1, ext_output_image_capture_source_manager_v1,
ext_idle_notifier_v1, ext_session_lock_manager_v1, ext_background_effect_manager_v1,
zcosmic_toplevel_manager_v1,           <-- o controlo das janelas
zcosmic_toplevel_info_v1, zcosmic_workspace_manager_v1, zcosmic_workspace_manager_v2,
zcosmic_output_manager_v1, zcosmic_overlap_notify_v1, zcosmic_keyboard_layout_manager_v1,
zwlr_output_manager_v1, zwlr_output_power_manager_v1, zwlr_layer_shell_v1,
zwlr_data_control_manager_v1, zwp_* (os do wayland-protocols unstable),
wp_pointer_warp_v1, wp_presentation, wp_fractional_scale_manager_v1, ...
```

### 8.3 O X11 e o XWayland não são a resposta

O `xprop`, o `xdotool` e o `wmctrl` continuam a funcionar numa sessão X11, e
o XWayland expõe as janelas X11. Mas:

- O XWayland só conhece as janelas **X11**. Numa sessão Wayland pura, um
  cliente nativo não aparece lá.
- Na caixa, o Xwayland é o do gamescope (rootless) e reporta uma lista de
  clientes **vazia** — é o que o `AGENTS.md` regista.
- Um programa que resolve a enumeração por X11 deixa de funcionar no
  momento em que a sessão passa a Wayland, e é exactamente o que a caixa
  é.

O caminho Wayland-native é o certo. O X11 fica como o *outro* backend, para
as máquinas que ainda são X11 — não como o universal.

### 8.4 Listar não é controlar

`ext-foreign-toplevel-list-v1` **não tem um único request de acção**. Com ele
o programa sabe que a janela existe, como se chama e qual é a sua aplicação,
e nada mais. Não a pode activar, nem fechar, nem mover.

Para isso é preciso um dos protocolos de gestão:

- `zwlr-foreign-toplevel_management_v1` nos compositores **wlroots**
  (Sway, Hyprland, Wayfire, riverlab, ...). Traz `activate`, `close`,
  `minimize`, `unmaximize`, `maximize`, `move`, `resize`, e os eventos de
  estado (`activated`, `fullscreen`, `minimized`) e de output.
- `zcosmic_toplevel_manager_v1` no **COSMIC**, que é o compositor da caixa.
  Traz `close`, `activate`, `set_maximized`/`unset_maximized`,
  `set_minimized`/`unset_minimized`, `set_fullscreen`/`unset_fullscreen`,
  `set_rectangle`, `set_sticky`/`unset_sticky`.

O `zcosmic_toplevel_info_v1` acrescenta ao que o `ext-` dá: os eventos
`state` (maximizado, minimizado, em ecrã inteiro) e `geometry` (a posição e
o tamanho da janela), além de `output_enter`/`output_leave` e
`workspace_enter`/`workspace_leave`.

**Estes protocolos não são portáveis entre compositores.** É a tensão
permanente do Wayland: a listagem pode ser padronizada, o controlo não, e
cada compositor faz as suas.

### 8.5 O `identifier` é estável, mas só enquanto a janela vive

O protocolo é explícito: o identificador *"is only valid as long as the
toplevel is mapped"*, não pode ser reutilizado, e deve incluir um valor de
geração opaco. Serve para ligar duas handles à mesma janela, dentro ou
entre processos. Não serve como identificador permanente de uma aplicação.

### 8.6 Um protocolo que não está no `wayland-protocols`

Se um dia for preciso um protocolo que o crate não traz (o
`zcosmic_toplevel_manager_v1` é um caso), o caminho é o `wayland-scanner`:

```toml
[build-dependencies]
wayland-scanner = "0.31"
```

```rust
// build.rs
fn main() {
    wayland_scanner::generate_client_code("protocols/cosmic-toplevel-management-v1.xml");
}
```

O XML do protocolo é a *especificação*; o `cosmic-protocols` (o crate) é
**GPL-3.0**, e esta árvore é **Apache 2.0**. A mesma regra do AHK_X11
aplica-se: o protocolo pode ser implementado, o código do outro projecto
não pode ser copiado. Gerar as ligações a partir do XML com o
`wayland-scanner` é escrever código nosso, e é o que se deve fazer.

---

## 9. O que isto dá ao runtime

O `AGENTS.md` (§7 e §23) diz que `WinExist` e `WinActive` respondem
*"unknown"* na caixa. A medição de 2026-10-28 diz que **essa recusa já não
é a única resposta possível**: o compositor da caixa publica a listagem.

O caminho para o tree, quando chegar a hora:

1. Uma capacidade nova em `ahk::desktop` — `Windows` já existe e já é
   recusado; o que falta é a implementação de `window_exists`,
   `window_active` e `window_title` por detrás do
   `ext-foreign-toplevel-list-v1`, com um `foreign_toplevel_list` no
   `ahk-box`.
2. Um `ahk-box` que ligue ao Wayland. Isto **não** é o `ahk-box` de hoje: o
   crate não fala Wayland, e o compositor é outra coisa. Pode ser uma
   dependência nova (`wayland-client` + `wayland-protocols`), ou um
   subprocesso que fala com o compositor e responde em JSON.
3. O `Clipboard`, que hoje recusa por omissão do trait, e que a caixa
   resolve com `ext_data_control_manager_v1`.
4. A captura de **uma janela** — `ext_foreign_toplevel_image_capture_source_manager_v1`
   dá a imagem de uma janela específica, o que o `grim` de hoje não dá
   (o `grim` dá o ecrã inteiro).

E uma nota de método: o `AGENTS.md` diz que *"a question that cannot be
answered fails, with the reason"*. A recusa de `WinExist` foi correcta
**enquanto** não houvesse caminho. A medição de hoje é o caminho, e a
recusa passa a ser uma resposta errada.

---

## 10. As fontes

- O protocolo: <https://phil294.github.io/protocols/ext-foreign-toplevel-list-v1/>
  e <https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/ext-foreign-toplevel-list/ext-foreign-toplevel-list-v1.xml>
- O crate: `wayland-client` 0.31.15, `wayland-protocols` 0.32.13,
  `wayland-backend` 0.3.17, `wayland-scanner` 0.31.11, `rustix` 1.1.6.
- A caixa: 192.168.0.40, CachyOS, `cosmic-comp`, socket `wayland-1`.
  Build em `~/src/wl-toplevels`, com Rust 1.98.1.
- O registo: 2026-10-28.
