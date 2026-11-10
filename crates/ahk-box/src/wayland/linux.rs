//! The Wayland client of the box.
//!
//! The client keeps the connection to the compositor open, and the
//! windows it has. Every question refreshes the answer: the compositor is
//! asked again, and the events that came since the last question are
//! dispatched.
//!
//! The two roundtrips of the refresh have a reason. The first one brings
//! the list of the windows. The questions about the state of those
//! windows are asked while the list is being dispatched, and they only
//! reach the compositor on the next flush -- so the second roundtrip is
//! what brings the answer.
//!
//! The state of a window is another matter: the compositor says it in its
//! next refresh, which is a frame away, and not in the answer to the
//! question. Measured on 2026-10-29 on the box: the answer was there on
//! the second question, ten milliseconds after the first one. So the
//! refresh waits for the state, but not forever -- see `wait_for_states`.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use wayland_client::globals::{BindError, GlobalList, GlobalListContents, registry_queue_init};
use wayland_client::protocol::wl_output::{Event as OutputEvent, WlOutput};
use wayland_client::protocol::wl_registry::{self, WlRegistry};
use wayland_client::{Connection, Dispatch, EventQueue, Proxy, QueueHandle, event_created_child};
use wayland_protocols::ext::foreign_toplevel_list::v1::client::{
    ext_foreign_toplevel_handle_v1::{Event as HandleEvent, ExtForeignToplevelHandleV1},
    ext_foreign_toplevel_list_v1::{self, Event as ListEvent, ExtForeignToplevelListV1},
};

use ahk::desktop::Monitor;
use super::Window;
use super::protocols::client::{
    zcosmic_toplevel_handle_v1::{Event as CosmicEvent, ZcosmicToplevelHandleV1},
    zcosmic_toplevel_info_v1::{self, Event as CosmicInfoEvent, ZcosmicToplevelInfoV1},
};

/// The value of the state that says the window is the active one.
///
/// The states are the ones of `xdg-toplevel`: maximized, minimized,
/// activated, fullscreen and sticky.
const ACTIVATED: u32 = 2;

/// How long the client waits for the state of the windows.
///
/// The compositor says the state in its next refresh, and a refresh is a
/// frame away: on the box the answer came ten milliseconds after the
/// question. A quarter of a second is a lot of frames, and it is the
/// bound of a compositor that never answers.
const STATE_TIMEOUT: Duration = Duration::from_millis(250);

/// How long the client sleeps between the attempts.
///
/// The answer comes with a frame, so a sleep shorter than that only
/// spends the CPU.
const STATE_PACE: Duration = Duration::from_millis(5);

/// The client of the compositor.
pub struct Client {
    /// The queue that carries the events.
    queue: EventQueue<State>,
    /// What the compositor has said.
    state: State,
    /// The identifier this runtime gives each window, by the identifier of
    /// the compositor. It is kept for the life of the process, so that the id
    /// of a window does not change between two questions.
    window_ids: BTreeMap<String, i64>,
    /// The next id a window that was not seen before takes.
    next_window_id: i64,
}

impl Client {
    /// Open the connection and ask the compositor for the windows.
    pub fn open() -> Result<Self, String> {
        let conn = Connection::connect_to_env()
            .map_err(|error| format!("cannot connect to the Wayland display: {error}"))?;
        let (globals, queue) = registry_queue_init::<State>(&conn)
            .map_err(|error| format!("cannot read the registry of the compositor: {error}"))?;

        let mut client = Client {
            queue,
            state: State {
                globals: Some(globals),
                ..State::default()
            },
            window_ids: BTreeMap::new(),
            // The zero is the window that is not there, and no window takes
            // it: the ids of the windows count from one.
            next_window_id: 1,
        };
        client.bind();
        client.refresh()?;
        Ok(client)
    }

    /// The monitors the compositor has.
    ///
    /// The compositor publishes one `wl_output` for each monitor, and the
    /// events of it say where the monitor is and how many pixels it has. The
    /// name of the output is the name the compositor gives it -- "HDMI-A-1"
    /// on the box -- and it is the name that `MonitorGetName` answers in the
    /// place of the "\\.\DISPLAY1" of the binary.
    ///
    /// Measured on the box on 2026-11-09: the compositor published one
    /// output, `HDMI-A-1`, at 0,0, with the mode 1920x1080.
    pub fn monitors(&mut self) -> Result<Vec<Monitor>, String> {
        self.refresh()?;
        if let Some(reason) = self.state.outputs_reason.clone() {
            return Err(reason);
        }
        Ok(self.state.monitors())
    }

    /// The windows the compositor has.
    pub fn windows(&mut self) -> Result<Vec<Window>, String> {
        self.refresh()?;
        if let Some(reason) = self.state.listing.clone() {
            return Err(reason);
        }
        Ok(self.state.windows())
    }

    /// The windows of the compositor, with the id this runtime gives each of
    /// them.
    ///
    /// The id is the identifier of this runtime, and not a handle of Windows:
    /// it is stable for the life of the window, and a number that no window
    /// has is a window that is not there.
    pub fn windows_with_ids(&mut self) -> Result<Vec<(i64, Window)>, String> {
        let windows = self.windows()?;
        Ok(windows
            .into_iter()
            .map(|window| (self.the_id_of(&window), window))
            .collect())
    }

    /// The window that has the focus.
    pub fn active(&mut self) -> Result<Option<Window>, String> {
        self.refresh()?;
        if let Some(reason) = self.state.listing.clone() {
            return Err(reason);
        }
        if let Some(reason) = self.state.focus.clone() {
            return Err(reason);
        }
        // The state of a window is what says that it has the focus, and
        // the compositor says it in its next refresh.
        self.wait_for_states()?;
        // A window whose state did not arrive leaves the question open:
        // the window that has the focus may be one of them.
        if let Some(reason) = self.state.pending_states() {
            return Err(reason);
        }
        Ok(self.state.active())
    }

    /// Ask the compositor for the protocols of the windows.
    fn bind(&mut self) {
        let Some(globals) = self.state.globals.take() else {
            return;
        };
        let handle = self.queue.handle();

        // The listing of the windows: the portable one.
        match globals.bind::<ExtForeignToplevelListV1, _, _>(&handle, 1..=1, ()) {
            Ok(list) => self.state.list = Some(list),
            Err(BindError::NotPresent) => {
                self.state.listing = Some(
                    "the compositor does not publish ext_foreign_toplevel_list_v1, so its windows \
                     cannot be listed through Wayland."
                        .to_string(),
                );
            }
            Err(BindError::UnsupportedVersion) => {
                self.state.listing = Some(
                    "the compositor publishes ext_foreign_toplevel_list_v1, but not the version \
                     this runtime speaks."
                        .to_string(),
                );
            }
        }

        // The information of COSMIC: it says which window is the active
        // one. The request that asks for it exists since the version 2.
        match globals.bind::<ZcosmicToplevelInfoV1, _, _>(&handle, 2..=3, ()) {
            Ok(info) => self.state.info = Some(info),
            Err(BindError::NotPresent) => {
                self.state.focus = Some(
                    "the compositor does not publish zcosmic_toplevel_info_v1, so it can list \
                     its windows but cannot say which one has the focus."
                        .to_string(),
                );
            }
            Err(BindError::UnsupportedVersion) => {
                self.state.focus = Some(
                    "the compositor publishes zcosmic_toplevel_info_v1, but not the version 2 \
                     that says which window is the active one."
                        .to_string(),
                );
            }
        }

        // The monitors of the compositor: one `wl_output` for each of them.
        // The listing of the globals is walked, because a bind by type takes
        // only the first output of a compositor that has two.
        let outputs: Vec<(u32, u32)> = globals
            .contents()
            .clone_list()
            .into_iter()
            .filter(|global| global.interface == WlOutput::interface().name)
            .map(|global| (global.name, global.version))
            .collect();
        if outputs.is_empty() {
            self.state.outputs_reason = Some(
                "the compositor does not publish wl_output, so its monitors cannot be read."
                    .to_string(),
            );
        }
        for (name, version) in outputs {
            // The name and the description of an output are the version 4
            // of the protocol: a compositor that speaks an older one has the
            // geometry and the mode, and no name.
            let output: WlOutput = globals.registry().bind(name, version.min(4), &handle, ());
            self.state
                .outputs
                .insert(output.id().protocol_id(), OutputState::default());
            self.state.output_proxies.push(output);
        }

        self.state.globals = Some(globals);
    }

    /// Ask the compositor what there is, and wait for the answer.
    /// The id of a window: the one it already has, and a new one when it was
    /// not seen before.
    ///
    /// The key of a window is the identifier the compositor gives it, which
    /// is stable for the life of the window. A compositor that gives none
    /// leaves the application id and the title, which is the best the
    /// listing has: the id of such a window may change when its title does.
    fn the_id_of(&mut self, window: &Window) -> i64 {
        let key = window
            .identifier
            .clone()
            .unwrap_or_else(|| format!("{}|{}", window.app_id, window.title));
        if let Some(id) = self.window_ids.get(&key) {
            return *id;
        }
        let id = self.next_window_id;
        self.next_window_id += 1;
        self.window_ids.insert(key, id);
        id
    }

    fn refresh(&mut self) -> Result<(), String> {
        self.roundtrip()?;
        self.roundtrip()
    }

    /// Wait for the state of the windows that the compositor has not said
    /// yet.
    ///
    /// The compositor says the state of a window in its next refresh, and
    /// a refresh is a frame away. The wait is bounded: a compositor that
    /// never answers must not hold the script forever.
    fn wait_for_states(&mut self) -> Result<(), String> {
        let deadline = Instant::now() + STATE_TIMEOUT;
        while self.state.has_pending_states() && Instant::now() < deadline {
            std::thread::sleep(STATE_PACE);
            self.roundtrip()?;
        }
        Ok(())
    }

    /// One roundtrip: the compositor answers when it has processed
    /// everything that was sent before.
    fn roundtrip(&mut self) -> Result<(), String> {
        self.queue
            .roundtrip(&mut self.state)
            .map_err(|error| format!("the compositor did not answer: {error}"))?;
        Ok(())
    }
}

/// What the compositor has said.
#[derive(Debug, Default)]
struct State {
    /// The globals of the compositor, kept for the ones that arrive late.
    globals: Option<GlobalList>,
    /// The windows, by the id of the handle of the listing.
    toplevels: BTreeMap<u32, Toplevel>,
    /// The handles of the listing. They are kept alive on purpose: a
    /// proxy that is dropped destroys the object on the server, and a
    /// handle that is destroyed never tells anything else.
    handles: Vec<ExtForeignToplevelHandleV1>,
    /// The handles of COSMIC, by the id of the handle of the listing they
    /// belong to.
    cosmic: BTreeMap<u32, u32>,
    /// The handles of COSMIC, kept alive for the same reason.
    cosmic_handles: Vec<ZcosmicToplevelHandleV1>,
    /// The object of the listing.
    list: Option<ExtForeignToplevelListV1>,
    /// The object of the information of COSMIC.
    info: Option<ZcosmicToplevelInfoV1>,
    /// Why the windows cannot be listed.
    listing: Option<String>,
    /// Why the active window cannot be told.
    focus: Option<String>,
    /// The monitors the compositor published, by the id of their output.
    outputs: BTreeMap<u32, OutputState>,
    /// The outputs themselves, kept alive on purpose: a proxy that is
    /// dropped destroys the object on the server, and a monitor that is
    /// destroyed never tells its name.
    output_proxies: Vec<WlOutput>,
    /// Why the monitors cannot be read.
    outputs_reason: Option<String>,
}

impl State {
    /// The windows, in the order of their handles.
    fn windows(&self) -> Vec<Window> {
        self.toplevels.values().map(Toplevel::window).collect()
    }

    /// The window that has the focus.
    fn active(&self) -> Option<Window> {
        self.toplevels
            .values()
            .find(|toplevel| toplevel.active)
            .map(Toplevel::window)
    }

    /// The monitors, in the order the compositor published them.
    ///
    /// A monitor with no mode yet -- the compositor has not said how many
    /// pixels it has -- is not a rectangle, and it is not answered: a monitor
    /// of zero pixels is a wrong answer, and not a monitor.
    fn monitors(&self) -> Vec<Monitor> {
        self.outputs
            .values()
            .filter(|output| output.width > 0 && output.height > 0)
            .map(OutputState::monitor)
            .collect()
    }

    /// Whether the compositor still has to say the state of a window.
    fn has_pending_states(&self) -> bool {
        self.toplevels.values().any(|toplevel| !toplevel.state_seen)
    }

    /// Why the state of a window is not known.
    fn pending_states(&self) -> Option<String> {
        let waiting = self
            .toplevels
            .values()
            .filter(|toplevel| !toplevel.state_seen)
            .count();
        if waiting == 0 {
            return None;
        }
        Some(format!(
            "the compositor did not say the state of {waiting} of the windows, so the window that \
             has the focus is not known."
        ))
    }
}

/// What is known about one window.
#[derive(Debug, Default)]
struct Toplevel {
    /// The identifier of the window, when the compositor gives one.
    identifier: Option<String>,
    /// The title, as the compositor confirmed it.
    title: String,
    /// The application id, as the compositor confirmed it.
    app_id: String,
    /// The title that arrived and waits for the `done` that confirms it.
    pending_title: Option<String>,
    /// The application id that arrived and waits for its `done`.
    pending_app_id: Option<String>,
    /// Whether the window is the active one.
    active: bool,
    /// Whether the compositor has said the state of the window.
    state_seen: bool,
}

impl Toplevel {
    /// The window, as the rest of the crate sees it.
    fn window(&self) -> Window {
        Window {
            identifier: self.identifier.clone(),
            title: self.title.clone(),
            app_id: self.app_id.clone(),
            active: self.active,
        }
    }
}

/// What is known about one monitor of the compositor.
#[derive(Debug, Default)]
struct OutputState {
    /// The name the compositor gives the output: "HDMI-A-1".
    name: String,
    /// The description of the output: the make and the model of it.
    description: String,
    /// The left edge of the monitor, in the coordinates of the session.
    x: i64,
    /// The top edge of the monitor, in the coordinates of the session.
    y: i64,
    /// The number of the columns of the current mode.
    width: i64,
    /// The number of the rows of the current mode.
    height: i64,
    /// Whether the mode the compositor has said is the current one.
    current: bool,
}

impl OutputState {
    /// The monitor, as the rest of the crate sees it.
    ///
    /// The rectangle of the monitor is the geometry of the output and the size
    /// of its current mode. The right and the bottom are one past the last
    /// column and the last row, which is the rectangle the binary of v2
    /// answers: measured on it on 2026-11-09, the monitor of a screen of 3840
    /// by 2160 answered 0, 0, 3840, 2160.
    fn monitor(&self) -> Monitor {
        Monitor {
            name: if self.name.is_empty() {
                self.description.clone()
            } else {
                self.name.clone()
            },
            left: self.x,
            top: self.y,
            right: self.x + self.width,
            bottom: self.y + self.height,
        }
    }

    /// Take a mode of the compositor.
    ///
    /// The compositor says the modes of the output, and the flags say which
    /// one is the current one. A mode that arrives before the current one is
    /// kept until the current one comes: a compositor that never says which
    /// one is current is answered with the last mode it said, which is its
    /// own order.
    fn take_mode(&mut self, flags: u32, width: i32, height: i32) {
        // The flag of `wl_output.mode` that says the mode is the current one.
        const CURRENT: u32 = 1;
        if flags & CURRENT != 0 {
            self.width = width as i64;
            self.height = height as i64;
            self.current = true;
        } else if !self.current {
            self.width = width as i64;
            self.height = height as i64;
        }
    }
}

/// One monitor: where it is, how many pixels it has, and its name.
impl Dispatch<WlOutput, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &WlOutput,
        event: OutputEvent,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        let Some(output) = state.outputs.get_mut(&proxy.id().protocol_id()) else {
            return;
        };
        match event {
            OutputEvent::Geometry { x, y, .. } => {
                output.x = x as i64;
                output.y = y as i64;
            }
            OutputEvent::Mode {
                flags,
                width,
                height,
                ..
            } => output.take_mode(flags.into(), width, height),
            OutputEvent::Name { name } => output.name = name,
            OutputEvent::Description { description } => output.description = description,
            // The scale and the transform of the output are not the
            // rectangle of the monitor in the coordinates of the session.
            _ => {}
        }
    }
}

/// The registry: the globals of the compositor.
impl Dispatch<WlRegistry, GlobalListContents> for State {
    fn event(
        state: &mut Self,
        proxy: &WlRegistry,
        event: wl_registry::Event,
        _data: &GlobalListContents,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        // The globals that are there when the client starts are bound by
        // the client itself. A compositor may publish one later, and it
        // is bound here.
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
        let list: ExtForeignToplevelListV1 = proxy.bind(name, version.min(1), qh, ());
        state.list = Some(list);
    }
}

/// The listing: the windows that are there, and the ones that come.
impl Dispatch<ExtForeignToplevelListV1, ()> for State {
    fn event(
        state: &mut Self,
        _proxy: &ExtForeignToplevelListV1,
        event: ListEvent,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            ListEvent::Toplevel { toplevel } => {
                let id = toplevel.id().protocol_id();
                state.toplevels.entry(id).or_default();
                state.handles.push(toplevel.clone());

                // Ask COSMIC what it knows about this window: the state
                // is what says whether the window is the active one.
                let cosmic = state
                    .info
                    .as_ref()
                    .map(|info| info.get_cosmic_toplevel(&toplevel, qh, ()));
                if let Some(cosmic) = cosmic {
                    state.cosmic.insert(cosmic.id().protocol_id(), id);
                    state.cosmic_handles.push(cosmic);
                }
            }
            ListEvent::Finished => {}
            // The generated enum is `#[non_exhaustive]`: an event that a
            // later version of the protocol adds lands here.
            _ => {}
        }
    }

    // The `toplevel` event creates the handle, and a child object needs
    // its user data before it can be used.
    event_created_child!(State, ExtForeignToplevelListV1, [
        ext_foreign_toplevel_list_v1::EVT_TOPLEVEL_OPCODE => (ExtForeignToplevelHandleV1, ()),
    ]);
}

/// The handle of the listing: what a window is.
impl Dispatch<ExtForeignToplevelHandleV1, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &ExtForeignToplevelHandleV1,
        event: HandleEvent,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        let id = proxy.id().protocol_id();
        let Some(toplevel) = state.toplevels.get_mut(&id) else {
            // A handle of a window that was closed before we got here.
            return;
        };
        let mut closed = false;

        match event {
            HandleEvent::Identifier { identifier } => {
                // The identifier is sent once, when the handle is made.
                toplevel.identifier = Some(identifier);
            }
            HandleEvent::Title { title } => {
                // The title is not applied until the `done` confirms it.
                toplevel.pending_title = Some(title);
            }
            HandleEvent::AppId { app_id } => {
                toplevel.pending_app_id = Some(app_id);
            }
            HandleEvent::Done => {
                if let Some(title) = toplevel.pending_title.take() {
                    toplevel.title = title;
                }
                if let Some(app_id) = toplevel.pending_app_id.take() {
                    toplevel.app_id = app_id;
                }
            }
            HandleEvent::Closed => {
                // The window is gone, and the compositor will not send
                // anything else on this handle.
                closed = true;
            }
            _ => {}
        }

        if closed {
            state.toplevels.remove(&id);
        }
    }
}

/// The manager of COSMIC: it holds the handles that carry the state.
impl Dispatch<ZcosmicToplevelInfoV1, ()> for State {
    fn event(
        _state: &mut Self,
        _proxy: &ZcosmicToplevelInfoV1,
        _event: CosmicInfoEvent,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        // The handles are asked for, one by one, when the listing arrives.
    }

    // A compositor that speaks the version 1 of the protocol sends the
    // handles itself, and a handle that arrives by an event needs its
    // user data too.
    event_created_child!(State, ZcosmicToplevelInfoV1, [
        zcosmic_toplevel_info_v1::EVT_TOPLEVEL_OPCODE => (ZcosmicToplevelHandleV1, ()),
    ]);
}

/// The handle of COSMIC: the state of a window.
impl Dispatch<ZcosmicToplevelHandleV1, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &ZcosmicToplevelHandleV1,
        event: CosmicEvent,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        let Some(id) = state.cosmic.get(&proxy.id().protocol_id()).copied() else {
            return;
        };
        let Some(toplevel) = state.toplevels.get_mut(&id) else {
            return;
        };

        if let CosmicEvent::State { state: states } = event {
            // The states are numbers, four bytes each, and the array
            // carries its length in bytes.
            toplevel.active = states.chunks_exact(4).any(|state| {
                u32::from_ne_bytes([state[0], state[1], state[2], state[3]]) == ACTIVATED
            });
            // The compositor has said the state of this window.
            toplevel.state_seen = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The id of a window is the same from one question to the next, and no
    /// window takes the zero that says "no window".
    ///
    /// The test needs the session of the box -- the compositor is what lists
    /// the windows -- and it stands aside when there is none, which is the
    /// way the tests of this crate are run over ssh.
    #[test]
    fn the_ids_of_the_windows_are_the_same_from_one_question_to_the_next() {
        if std::env::var_os("WAYLAND_DISPLAY").is_none() {
            return;
        }
        let mut client = Client::open().expect("the compositor is there");
        let first = client.windows_with_ids().expect("the windows are listed");
        let second = client.windows_with_ids().expect("the windows are listed");
        assert_eq!(first.len(), second.len());
        for ((id, window), (other, other_window)) in first.iter().zip(second.iter()) {
            assert_eq!(id, other, "the id of {:?} moved", window.title);
            assert_eq!(window.title, other_window.title);
            assert_ne!(*id, 0, "no window takes the zero of no window");
        }
    }
}