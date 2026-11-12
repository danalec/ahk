//! An X11 desktop, driven through Xlib and XTEST resolved at runtime.
//!
//! WHY: the uinput keyboard and the absolute pointer in `desktop.rs` only
//! work on Linux with `/dev/uinput`, and uinput injects events *below* the
//! display server, so it cannot follow X11 focus. FreeBSD has no uinput at
//! all. XTEST is the one interface that reaches an X server as a real
//! input device on both hosts and honours the server's focus, so an X11
//! session (Xorg, XWayland or FreeBSD X11) needs this backend.
//!
//! The backend is **additive**. Nothing in `desktop.rs` is changed: the
//! caller selects it with `YOKO_DESKTOP=x11` (see `crate::desktop_kind`),
//! asks `crate::x11_available()`, and uses the `Keyboard`/`Pointer`/`Screen`/
//! `Windows` implementations here. `DISPLAY` comes from `YOKO_DISPLAY` or
//! `DISPLAY`, defaulting to `:0`.
//!
//! No X11 library is linked at build time. `libX11.so.6` and `libXtst.so.6`
//! are opened with `dlopen` on the first call and every function pointer is
//! resolved with `dlsym`, so a machine without X11 still builds, links and
//! runs — every method then answers a clear `Err` instead of panicking.
//! `XDestroyImage` and `XGetPixel` are Xlib **macros**, not symbols, so they
//! are deliberately absent from the table: the image is released with
//! `XFree` and its pixels are read straight out of `XImage->data`.

use std::ffi::{CStr, CString, c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
use std::sync::atomic::{AtomicI8, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, TitleMatch, WindowAnswer, Windows,
};

// ---------------------------------------------------------------------------
// The FFI seam: every `unsafe` in this file lives inside this module.
// ---------------------------------------------------------------------------

/// The Xlib / XTEST calls, resolved once at runtime.
///
/// WHY one table, one state: the two libraries are opened together and a
/// missing library or symbol must make *every* wrapper answer "absent", not
/// just the one that happened to be called first. `STATE` gates the load so
/// only the thread that wins `0 -> 2` writes `PTRS`, and readers spin until
/// it leaves `2`, so there is no torn read.
mod ffi {
    use super::*;

    /// The runtime soname of Xlib, NUL-terminated for `dlopen`.
    const LIBX11: &[u8] = b"libX11.so.6\0";
    /// The runtime soname of the XTEST extension library.
    const LIBXTST: &[u8] = b"libXtst.so.6\0";
    /// `RTLD_LAZY`; `RTLD_LOCAL` is zero on both Linux and FreeBSD.
    const RTLD_LAZY: c_int = 1;
    /// The first index in `SYMBOLS` that belongs to libXtst rather than libX11.
    pub(super) const LIBXTST_FROM: usize = 21;
    /// The number of resolved symbols.
    pub(super) const SYMBOL_COUNT: usize = 24;

    /// The symbols, in a FIXED order: entry `i` belongs to wrapper `i`.
    ///
    /// `XDestroyImage` and `XGetPixel` are macros and are therefore NOT
    /// here. Each name is a NUL-terminated C string, because `dlsym` takes
    /// one.
    #[rustfmt::skip]
    pub(super) const SYMBOLS: [&[u8]; SYMBOL_COUNT] = [
        b"XOpenDisplay\0",          // 0  Xlib
        b"XCloseDisplay\0",         // 1
        b"XDefaultRootWindow\0",    // 2
        b"XStringToKeysym\0",       // 3
        b"XKeysymToKeycode\0",      // 4
        b"XFlush\0",                // 5
        b"XGetImage\0",             // 6
        b"XFree\0",                 // 7
        b"XGetWindowProperty\0",    // 8
        b"XInternAtom\0",           // 9
        b"XQueryTree\0",            // 10
        b"XFetchName\0",            // 11
        b"XGetWindowAttributes\0",  // 12
        b"XRaiseWindow\0",          // 13
        b"XSetInputFocus\0",        // 14
        b"XPending\0",              // 15
        b"XNextEvent\0",            // 16
        b"XLookupString\0",         // 17
        b"XKeysymToString\0",       // 18
        b"XSendEvent\0",            // 19
        b"XTranslateCoordinates\0", // 20

        b"XTestFakeKeyEvent\0",     // 21 Xtst
        b"XTestFakeButtonEvent\0",  // 22
        b"XTestFakeMotionEvent\0",  // 23
    ];

    const _: () = assert!(SYMBOLS.len() == SYMBOL_COUNT);
    const _: () = assert!(LIBXTST_FROM < SYMBOL_COUNT);
    const _: () = assert!(std::mem::size_of::<*mut c_void>() == std::mem::size_of::<usize>());

    /// The resolved pointers. Written once by the `0 -> 2` winner.
    static mut PTRS: [*mut c_void; SYMBOL_COUNT] = [std::ptr::null_mut(); SYMBOL_COUNT];
    /// Load state: 0 = not tried, 2 = loading, 1 = ready, -1 = failed.
    static STATE: AtomicI8 = AtomicI8::new(0);

    /// Whether both libraries and every symbol resolved, resolving once.
    ///
    /// WHY: `crate::x11_available()` must be able to answer without a display
    /// and without repeating the `dlopen` on every call.
    pub(super) fn available() -> bool {
        ensure_loaded();
        STATE.load(Ordering::Acquire) == 1
    }

    /// Resolve every symbol once, on the first call.
    fn ensure_loaded() {
        let state = STATE.load(Ordering::Acquire);
        if state == 1 || state == -1 {
            return;
        }
        if STATE
            .compare_exchange(0, 2, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            while STATE.load(Ordering::Acquire) == 2 {
                std::hint::spin_loop();
            }
            return;
        }
        let ok = unsafe { load_all() };
        STATE.store(if ok { 1 } else { -1 }, Ordering::Release);
    }

    /// `dlopen` both sonames and resolve the whole table into `PTRS`.
    ///
    /// # Safety
    /// Only the `STATE == 2` winner calls this, once, before `STATE` is
    /// published as ready.
    unsafe fn load_all() -> bool {
        unsafe {
            let xlib = dlopen(LIBX11.as_ptr().cast(), RTLD_LAZY);
            let xtst = dlopen(LIBXTST.as_ptr().cast(), RTLD_LAZY);
            if xlib.is_null() || xtst.is_null() {
                return false;
            }
            for (i, name) in SYMBOLS.iter().enumerate() {
                let handle = if i < LIBXTST_FROM { xlib } else { xtst };
                PTRS[i] = dlsym(handle, name.as_ptr().cast());
                if PTRS[i].is_null() {
                    // Leave the libraries loaded: a wrapper may still be
                    // called and must answer zero, not jump into an unloaded
                    // library through a stale pointer.
                    return false;
                }
            }
            // Deliberately no dlclose: the pointers live as long as the process.
            true
        }
    }

    /// The pointer for symbol `i`, or null when X11 is absent.
    fn sym(i: usize) -> *mut c_void {
        ensure_loaded();
        if STATE.load(Ordering::Acquire) != 1 {
            return std::ptr::null_mut();
        }
        unsafe { PTRS[i] }
    }

    /// Generate one safe wrapper around a resolved Xlib/XTEST call.
    ///
    /// WHY safe: all `unsafe` is confined to this generated body, so the
    /// rest of the backend never writes an `unsafe` block. A missing library
    /// answers the zero value, which the caller reports as a clear `Err`.
    macro_rules! xfn {
        ($idx:expr, $name:ident ( $($a:ident : $t:ty),* ) -> $r:ty) => {
            #[allow(non_snake_case, clippy::too_many_arguments)]
            #[doc = concat!("The `", stringify!($name), "` X11 call, resolved at runtime.")]
            pub(super) fn $name($($a: $t),*) -> $r {
                const _: () = assert!(($idx as usize) < SYMBOL_COUNT);
                let p = sym($idx);
                if p.is_null() {
                    return unsafe { std::mem::zeroed::<$r>() };
                }
                unsafe {
                    let f: unsafe extern "C" fn($($t),*) -> $r = std::mem::transmute(p);
                    f($($a),*)
                }
            }
        };
    }

    #[rustfmt::skip]
    xfn!(0, XOpenDisplay(name: *const c_char) -> *mut c_void);
    #[rustfmt::skip]
    xfn!(1, XCloseDisplay(d: *mut c_void) -> c_int);
    #[rustfmt::skip]
    xfn!(2, XDefaultRootWindow(d: *mut c_void) -> c_ulong);
    #[rustfmt::skip]
    xfn!(3, XStringToKeysym(name: *const c_char) -> c_ulong);
    #[rustfmt::skip]
    xfn!(4, XKeysymToKeycode(d: *mut c_void, ks: c_ulong) -> c_uchar);
    #[rustfmt::skip]
    xfn!(5, XFlush(d: *mut c_void) -> c_int);
    #[rustfmt::skip]
    xfn!(6, XGetImage(d: *mut c_void, drawable: c_ulong, x: c_int, y: c_int, w: c_uint, h: c_uint, plane_mask: c_ulong, format: c_int) -> *mut c_void);
    #[rustfmt::skip]
    xfn!(7, XFree(p: *mut c_void) -> c_int);
    #[rustfmt::skip]
    xfn!(8, XGetWindowProperty(d: *mut c_void, w: c_ulong, property: c_ulong, long_offset: c_long, long_length: c_long, delete: c_int, req_type: c_ulong, actual_format: *mut c_int, actual_type: *mut c_ulong, nitems: *mut c_ulong, bytes_after: *mut c_ulong, prop: *mut *mut c_uchar) -> c_int);
    #[rustfmt::skip]
    xfn!(9, XInternAtom(d: *mut c_void, name: *const c_char, only_if_exists: c_int) -> c_ulong);
    #[rustfmt::skip]
    xfn!(10, XQueryTree(d: *mut c_void, w: c_ulong, root: *mut c_ulong, parent: *mut c_ulong, children: *mut *mut c_ulong, count: *mut c_uint) -> c_int);
    #[rustfmt::skip]
    xfn!(11, XFetchName(d: *mut c_void, w: c_ulong, name: *mut *mut c_char) -> c_int);
    #[rustfmt::skip]
    xfn!(12, XGetWindowAttributes(d: *mut c_void, w: c_ulong, attrs: *mut c_void) -> c_int);
    #[rustfmt::skip]
    xfn!(13, XRaiseWindow(d: *mut c_void, w: c_ulong) -> c_int);
    #[rustfmt::skip]
    xfn!(14, XSetInputFocus(d: *mut c_void, focus: c_ulong, revert_to: c_int, time: c_ulong) -> c_int);
    #[rustfmt::skip]
    xfn!(15, XPending(d: *mut c_void) -> c_int);
    #[rustfmt::skip]
    xfn!(16, XNextEvent(d: *mut c_void, event: *mut c_void) -> c_int);
    #[rustfmt::skip]
    xfn!(17, XLookupString(event: *const c_void, buffer: *mut c_char, bytes: c_int, keysym: *mut c_ulong, status: *mut c_void) -> c_int);
    #[rustfmt::skip]
    xfn!(18, XKeysymToString(ks: c_ulong) -> *const c_char);
    #[rustfmt::skip]
    xfn!(19, XSendEvent(d: *mut c_void, e: *mut c_void, propagate: c_int, mask: c_long, w: c_ulong) -> c_int);
    #[rustfmt::skip]
    xfn!(20, XTranslateCoordinates(d: *mut c_void, src: c_ulong, dst: c_ulong, src_x: c_int, src_y: c_int, dst_x: *mut c_int, dst_y: *mut c_int, child: *mut c_ulong) -> c_int);

    xfn!(21, XTestFakeKeyEvent(d: *mut c_void, keycode: c_uint, is_press: c_int, delay: c_ulong) -> c_int);
    #[rustfmt::skip]
    xfn!(22, XTestFakeButtonEvent(d: *mut c_void, button: c_uint, is_press: c_int, delay: c_ulong) -> c_int);
    #[rustfmt::skip]
    xfn!(23, XTestFakeMotionEvent(d: *mut c_void, screen: c_int, x: c_int, y: c_int, delay: c_ulong) -> c_int);
    /// The client message of the EWMH, as Xlib lays it out.
    ///
    /// The layout is the one of `XClientMessageEvent` on a 64-bit Linux:
    /// the `long` of the union is eight bytes, and the union is five of
    /// them. The padding of the C struct is the padding of this one.
    #[repr(C)]
    struct ClientMessage {
        kind: c_int,
        serial: c_ulong,
        send_event: c_int,
        display: *mut c_void,
        window: c_ulong,
        message_type: c_ulong,
        format: c_int,
        data: [c_long; 5],
    }

    /// The kind of the event of a client message.
    const CLIENT_MESSAGE: c_int = 33;
    /// The mask of the events that the root window wants to hear about.
    ///
    /// It is `SubstructureRedirectMask | SubstructureNotifyMask`.
    const SUBSTRUCTURE_MASK: c_long = 0x0018_0000;
    /// The type of the property of a list of atoms.
    const XA_ATOM: c_ulong = 4;

    /// Whether the window is above the others, when the server says.
    ///
    /// The state of a window is the property `_NET_WM_STATE`, a list of
    /// atoms, and `_NET_WM_STATE_ABOVE` among them is the always on top.
    /// `None` is the answer of a window that is not there.
    pub(super) fn always_on_top(d: *mut c_void, window: c_ulong) -> Option<bool> {
        let state = XInternAtom(d, CString::new("_NET_WM_STATE").ok()?.as_ptr(), 1);
        let above = XInternAtom(d, CString::new("_NET_WM_STATE_ABOVE").ok()?.as_ptr(), 1);
        if state == 0 || above == 0 {
            return None;
        }
        let mut format: c_int = 0;
        let mut actual_type: c_ulong = 0;
        let mut nitems: c_ulong = 0;
        let mut after: c_ulong = 0;
        let mut prop: *mut c_uchar = std::ptr::null_mut();
        let status = XGetWindowProperty(
            d,
            window,
            state,
            0,
            32,
            0,
            XA_ATOM,
            &mut format,
            &mut actual_type,
            &mut nitems,
            &mut after,
            &mut prop,
        );
        let mut found = false;
        if status == 0 && format == 32 && !prop.is_null() {
            let atoms = prop.cast::<c_ulong>();
            for index in 0..nitems as usize {
                if unsafe { *atoms.add(index) } == above {
                    found = true;
                }
            }
        }
        if !prop.is_null() {
            XFree(prop.cast());
        }
        if status != 0 {
            // The window is not there, and the server says so.
            return None;
        }
        Some(found)
    }

    /// Put the window above the others, or take it from there.
    ///
    /// The window manager is asked with the client message of the EWMH:
    /// the property `_NET_WM_STATE` of the root window carries the atom
    /// `_NET_WM_STATE_ABOVE` and the action, and the window manager is the
    /// one that changes the state of the window.
    pub(super) fn set_always_on_top(
        d: *mut c_void,
        root: c_ulong,
        window: c_ulong,
        on: bool,
    ) -> bool {
        let state = match CString::new("_NET_WM_STATE") {
            Ok(name) => XInternAtom(d, name.as_ptr(), 0),
            Err(_) => return false,
        };
        let above = match CString::new("_NET_WM_STATE_ABOVE") {
            Ok(name) => XInternAtom(d, name.as_ptr(), 0),
            Err(_) => return false,
        };
        if state == 0 || above == 0 {
            return false;
        }
        let mut event = ClientMessage {
            kind: CLIENT_MESSAGE,
            serial: 0,
            send_event: 1,
            display: d,
            window,
            message_type: state,
            format: 32,
            // The action is the first item: zero removes the state, one
            // adds it and two toggles it. The last item says that the
            // request comes from the application.
            data: [if on { 1 } else { 0 }, above as c_long, state as c_long, 1, 0],
        };
        XSendEvent(
            d,
            (&mut event as *mut ClientMessage).cast(),
            0,
            SUBSTRUCTURE_MASK,
            root,
        ) != 0
    }

    /// Whether the window is mapped and not iconified.
    ///
    /// This is the enabled state of a window of X11: a window that is not
    /// mapped is not there, and a window that is iconified is not on the
    /// screen.
    pub(super) fn window_is_enabled(d: *mut c_void, window: c_ulong) -> Option<bool> {
        // The attributes of a window are read into a buffer of the size of
        // `XWindowAttributes`, and the map state is its ninth field.
        #[repr(C)]
        struct Attributes {
            x: c_int,
            y: c_int,
            width: c_int,
            height: c_int,
            border_width: c_int,
            depth: c_int,
            visual: *mut c_void,
            root: c_ulong,
            class: c_int,
            bit_gravity: c_int,
            win_gravity: c_int,
            backing_store: c_int,
            backing_planes: c_ulong,
            backing_pixel: c_ulong,
            save_under: c_int,
            colormap: c_ulong,
            map_installed: c_int,
            map_state: c_int,
            all_event_masks: c_long,
            your_event_mask: c_long,
            do_not_propagate_mask: c_long,
            override_redirect: c_int,
            screen: *mut c_void,
        }
        let mut attributes = Attributes {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            border_width: 0,
            depth: 0,
            visual: std::ptr::null_mut(),
            root: 0,
            class: 0,
            bit_gravity: 0,
            win_gravity: 0,
            backing_store: 0,
            backing_planes: 0,
            backing_pixel: 0,
            save_under: 0,
            colormap: 0,
            map_installed: 0,
            map_state: 0,
            all_event_masks: 0,
            your_event_mask: 0,
            do_not_propagate_mask: 0,
            override_redirect: 0,
            screen: std::ptr::null_mut(),
        };
        if XGetWindowAttributes(d, window, (&mut attributes as *mut Attributes).cast()) == 0 {
            return None;
        }
        // The map state is 0 when the window is unmapped, 1 when it is
        // viewable and 2 when it is iconified.
        Some(attributes.map_state == 1)
    }

    // The libc dynamic loader, declared by hand (no `#[link]`, no crate).
    unsafe extern "C" {
        fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }

    /// Read a NUL-terminated string Xlib handed back, without taking it.
    pub(super) fn cstring(p: *const c_char) -> Option<String> {
        if p.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned())
    }

    /// `XGetWindowProperty`'s 32-bit first value, or `None` when absent.
    ///
    /// WHY: `_NET_ACTIVE_WINDOW` is a 32-bit property whose values Xlib
    /// stores in `unsigned long`s; only the low word is the window id.
    pub(super) fn property_first(prop: *mut c_uchar) -> Option<c_ulong> {
        if prop.is_null() {
            return None;
        }
        Some(unsafe { std::ptr::read_unaligned(prop.cast::<c_ulong>()) } & 0xffff_ffff)
    }

    /// The title of a window, or `None`; the name Xlib allocated is freed.
    pub(super) fn fetch_name(d: *mut c_void, w: c_ulong) -> Option<String> {
        let mut name: *mut c_char = std::ptr::null_mut();
        if XFetchName(d, w, &mut name) == 0 {
            return None;
        }
        let title = cstring(name);
        if !name.is_null() {
            XFree(name.cast());
        }
        title
    }

    /// The children of a window in the tree; the array Xlib allocated is freed.
    pub(super) fn query_tree(d: *mut c_void, w: c_ulong) -> Vec<c_ulong> {
        let mut root: c_ulong = 0;
        let mut parent: c_ulong = 0;
        let mut children: *mut c_ulong = std::ptr::null_mut();
        let mut count: c_uint = 0;
        if XQueryTree(d, w, &mut root, &mut parent, &mut children, &mut count) == 0 {
            return Vec::new();
        }
        let mut out = Vec::new();
        if !children.is_null() {
            for index in 0..count as usize {
                out.push(unsafe { *children.add(index) });
            }
            XFree(children.cast());
        }
        out
    }

    /// The id of the active window, from `_NET_ACTIVE_WINDOW` on the root.
    pub(super) fn active_window(d: *mut c_void, root: c_ulong) -> Option<c_ulong> {
        let atom = CString::new("_NET_ACTIVE_WINDOW").ok()?;
        let property = XInternAtom(d, atom.as_ptr(), 1);
        if property == 0 {
            return None;
        }
        let mut format: c_int = 0;
        let mut actual_type: c_ulong = 0;
        let mut nitems: c_ulong = 0;
        let mut after: c_ulong = 0;
        let mut prop: *mut c_uchar = std::ptr::null_mut();
        // AnyPropertyType is 0; one 32-bit item is one `long` of the reply.
        let status = XGetWindowProperty(
            d,
            root,
            property,
            0,
            1,
            0,
            0,
            &mut format,
            &mut actual_type,
            &mut nitems,
            &mut after,
            &mut prop,
        );
        let id = if status == 0 && format == 32 && nitems > 0 {
            property_first(prop)
        } else {
            None
        };
        if !prop.is_null() {
            XFree(prop.cast());
        }
        id.filter(|id| *id != 0)
    }

    /// The keysym's X11 name, or `None`; Xlib owns the returned string.
    pub(super) fn keysym_to_string(ks: c_ulong) -> Option<String> {
        cstring(XKeysymToString(ks))
    }

    /// One pixel of an `XImage`, read from `data` at the documented offsets.
    ///
    /// WHY: `XGetPixel` is a macro, so there is no symbol to resolve. The
    /// colour masks are Xlib's own and are scaled to eight bits, so a 24-
    /// and a 32-bit visual answer the same `#rrggbb`.
    pub(super) fn ximage_pixel(image: *mut c_void, x: c_int, y: c_int) -> Option<(u8, u8, u8)> {
        if image.is_null() {
            return None;
        }
        let layout = super::ximage_pixel_layout();
        unsafe {
            let base = image.cast::<u8>();
            let width = std::ptr::read_unaligned(base.add(layout.width).cast::<c_int>());
            let height = std::ptr::read_unaligned(base.add(layout.height).cast::<c_int>());
            let depth = std::ptr::read_unaligned(base.add(layout.depth).cast::<c_int>());
            let stride = std::ptr::read_unaligned(base.add(layout.bytes_per_line).cast::<c_int>());
            let bpp = std::ptr::read_unaligned(base.add(layout.bpp).cast::<c_int>());
            let data = std::ptr::read_unaligned(base.add(layout.data).cast::<*const u8>());
            if x < 0
                || y < 0
                || x >= width
                || y >= height
                || data.is_null()
                || bpp < 8
                || depth <= 0
            {
                return None;
            }
            let bytes = (bpp / 8) as usize;
            if bytes > 4 || stride <= 0 {
                return None;
            }
            let at = data.add(y as usize * stride as usize + x as usize * bytes);
            let pixel: usize = match bytes {
                1 => *at as usize,
                2 => std::ptr::read_unaligned(at.cast::<u16>()) as usize,
                3 => (*at as usize) | ((*at.add(1) as usize) << 8) | ((*at.add(2) as usize) << 16),
                _ => std::ptr::read_unaligned(at.cast::<u32>()) as usize,
            };
            let masks = [
                std::ptr::read_unaligned(base.add(layout.red_mask).cast::<c_ulong>()) as usize,
                std::ptr::read_unaligned(base.add(layout.green_mask).cast::<c_ulong>()) as usize,
                std::ptr::read_unaligned(base.add(layout.blue_mask).cast::<c_ulong>()) as usize,
            ];
            let mut channels = [0u8; 3];
            for (channel, mask) in channels.iter_mut().zip(masks) {
                *channel = if mask == 0 {
                    (pixel & 0xff) as u8
                } else {
                    let shift = mask.trailing_zeros() as usize;
                    let maximum = mask >> shift;
                    let value = (pixel & mask) >> shift;
                    (value.saturating_mul(255) / maximum.max(1)) as u8
                };
            }
            Some((channels[0], channels[1], channels[2]))
        }
    }

    /// Release an image from `XGetImage`.
    ///
    /// WHY: `XDestroyImage` is a macro, not a symbol. For an image Xlib made,
    /// its `destroy_image` frees `data` and then the struct, which is exactly
    /// these two `XFree` calls — so both halves are released without ever
    /// resolving a symbol that does not exist.
    pub(super) fn ximage_free(image: *mut c_void) {
        if image.is_null() {
            return;
        }
        let layout = super::ximage_pixel_layout();
        let data = unsafe {
            std::ptr::read_unaligned(image.cast::<u8>().add(layout.data).cast::<*mut c_void>())
        };
        XFree(image);
        if !data.is_null() {
            XFree(data);
        }
    }
}

// ---------------------------------------------------------------------------
// Pure helpers: no display, no library, no unsafe.
// ---------------------------------------------------------------------------

/// The LP64 byte offsets inside Xlib's `XImage` that this backend reads.
///
/// WHY: `XGetPixel` is a macro, so pixels have to be read from `data` by
/// hand. The offsets come from a `#[repr(C)]` mirror of Xlib's header rather
/// than from literals, so a target whose `unsigned long` is four bytes gets
/// its own correct offsets for the mask fields. The first twelve fields are
/// `int`s on every ABI, which is what the compile-time assertions below fix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XImageLayout {
    /// Offset of `int width`.
    pub width: usize,
    /// Offset of `int height`.
    pub height: usize,
    /// Offset of `char *data`.
    pub data: usize,
    /// Offset of `int depth`.
    pub depth: usize,
    /// Offset of `int bytes_per_line`.
    pub bytes_per_line: usize,
    /// Offset of `int bits_per_pixel`.
    pub bpp: usize,
    /// Offset of `unsigned long red_mask`.
    pub red_mask: usize,
    /// Offset of `unsigned long green_mask`.
    pub green_mask: usize,
    /// Offset of `unsigned long blue_mask`.
    pub blue_mask: usize,
}

/// The mirror of the leading fields of Xlib's `XImage`, used only for its
/// offsets. Xlib writes the whole struct; this mirror is never passed to it.
#[repr(C)]
#[allow(dead_code)]
struct XImageHeader {
    width: c_int,
    height: c_int,
    xoffset: c_int,
    format: c_int,
    data: *const c_char,
    byte_order: c_int,
    bitmap_unit: c_int,
    bitmap_bit_order: c_int,
    bitmap_pad: c_int,
    depth: c_int,
    bytes_per_line: c_int,
    bits_per_pixel: c_int,
    red_mask: c_ulong,
    green_mask: c_ulong,
    blue_mask: c_ulong,
}

const _: () = {
    assert!(std::mem::offset_of!(XImageHeader, width) == 0);
    assert!(std::mem::offset_of!(XImageHeader, height) == 4);
    assert!(std::mem::offset_of!(XImageHeader, data) == 16);
    assert!(
        std::mem::offset_of!(XImageHeader, depth)
            < std::mem::offset_of!(XImageHeader, bytes_per_line)
    );
    assert!(
        std::mem::offset_of!(XImageHeader, bytes_per_line)
            < std::mem::offset_of!(XImageHeader, bits_per_pixel)
    );
    assert!(
        std::mem::offset_of!(XImageHeader, bits_per_pixel)
            < std::mem::offset_of!(XImageHeader, red_mask)
    );
};

/// The byte offsets of the `XImage` fields the backend reads.
///
/// WHY: a pure function so callers can see the layout the FFI uses, and so
/// the unit test can pin the LP64 numbers without opening a display.
pub fn ximage_pixel_layout() -> XImageLayout {
    use std::mem::offset_of;
    XImageLayout {
        width: offset_of!(XImageHeader, width),
        height: offset_of!(XImageHeader, height),
        data: offset_of!(XImageHeader, data),
        depth: offset_of!(XImageHeader, depth),
        bytes_per_line: offset_of!(XImageHeader, bytes_per_line),
        bpp: offset_of!(XImageHeader, bits_per_pixel),
        red_mask: offset_of!(XImageHeader, red_mask),
        green_mask: offset_of!(XImageHeader, green_mask),
        blue_mask: offset_of!(XImageHeader, blue_mask),
    }
}

// LP64 is not the same as a 64-bit pointer: Windows x86_64 is LLP64, where
// `unsigned long` is four bytes. These are the Xlib offsets on the two hosts
// this module is compiled for, where a 64-bit pointer means a 64-bit `long`.
#[cfg(all(
    target_pointer_width = "64",
    any(target_os = "linux", target_os = "freebsd")
))]
const _: () = {
    assert!(std::mem::offset_of!(XImageHeader, depth) == 40);
    assert!(std::mem::offset_of!(XImageHeader, bytes_per_line) == 44);
    assert!(std::mem::offset_of!(XImageHeader, bits_per_pixel) == 48);
    assert!(std::mem::offset_of!(XImageHeader, red_mask) == 56);
    assert!(std::mem::offset_of!(XImageHeader, green_mask) == 64);
    assert!(std::mem::offset_of!(XImageHeader, blue_mask) == 72);
};

/// Whether the Xlib and XTEST libraries resolved.
///
/// WHY: `crate::x11_available()` asks this once, without opening a display,
/// so the caller can fall back to the uinput/Wayland desktop on a machine
/// that has no X11. `dlopen` happens exactly once, in the symbol table.
pub fn available() -> bool {
    ffi::available()
}

/// The display to open: `YOKO_DISPLAY`, else `DISPLAY`, else `:0`.
///
/// WHY: over SSH both variables are usually empty, and the local Xorg or
/// XWayland server is almost always `:0`. A caller that needs another one
/// sets `YOKO_DISPLAY` without disturbing the session's own `DISPLAY`.
pub fn display_name() -> String {
    for key in ["YOKO_DISPLAY", "DISPLAY"] {
        if let Ok(value) = std::env::var(key) {
            if !value.is_empty() {
                return value;
            }
        }
    }
    ":0".to_string()
}

/// The X11 KeySym spelling of an AutoHotkey key name.
///
/// WHY: XTEST takes a *keycode*, and `XStringToKeysym` takes an X11 name such
/// as `Return`, `Prior` or `Shift_L`. AHK spells the same keys `Enter`,
/// `PgUp` and `ctrl`. A single character keeps its case (`a` and `A` are
/// different keys in AHK); named keys are matched case-insensitively. The
/// answer is passed straight to `XStringToKeysym`, so an unknown name is
/// `None` rather than a guess.
pub fn keysym_name(name: &str) -> Option<&'static str> {
    if name.chars().count() == 1 {
        return name
            .chars()
            .next()
            .and_then(|c| character_stroke(c).map(|(sym, _)| sym));
    }
    let lowered = name.to_ascii_lowercase();
    Some(match lowered.as_str() {
        "return" | "enter" => "Return",
        "escape" | "esc" => "Escape",
        "tab" => "Tab",
        "backspace" | "bs" | "bksp" => "BackSpace",
        "space" | "spacebar" => "space",
        "up" => "Up",
        "down" => "Down",
        "left" => "Left",
        "right" => "Right",
        "home" => "Home",
        "end" => "End",
        "page_up" | "pageup" | "pgup" | "prior" => "Prior",
        "page_down" | "pagedown" | "pgdn" | "next" => "Next",
        "delete" | "del" => "Delete",
        "insert" | "ins" => "Insert",
        "f1" => "F1",
        "f2" => "F2",
        "f3" => "F3",
        "f4" => "F4",
        "f5" => "F5",
        "f6" => "F6",
        "f7" => "F7",
        "f8" => "F8",
        "f9" => "F9",
        "f10" => "F10",
        "f11" => "F11",
        "f12" => "F12",
        "ctrl" | "control" | "lctrl" | "ctrlleft" => "Control_L",
        "rctrl" | "ctrlright" => "Control_R",
        "shift" | "lshift" | "shiftleft" => "Shift_L",
        "rshift" | "shiftright" => "Shift_R",
        "alt" | "lalt" | "altleft" => "Alt_L",
        "ralt" | "altright" => "Alt_R",
        "super" | "win" | "lwin" | "meta" => "Super_L",
        "rwin" => "Super_R",
        _ => return None,
    })
}

/// The KeySym name of a modifier, for `press_key`'s modifier list.
///
/// WHY: the runtime hands modifiers as AHK names (`ctrl`, `shift`, `alt`,
/// `super`); XTEST needs the keycode of a *specific* modifier key, so the
/// name is turned into a left/right keysym first.
pub fn modifier_keysym(name: &str) -> Option<&'static str> {
    match name.to_ascii_lowercase().as_str() {
        "ctrl" | "control" | "lctrl" | "lcontrol" | "ctrlleft" => Some("Control_L"),
        "rctrl" | "rcontrol" | "ctrlright" => Some("Control_R"),
        "shift" | "lshift" | "shiftleft" => Some("Shift_L"),
        "rshift" | "shiftright" => Some("Shift_R"),
        "alt" | "lalt" | "altleft" => Some("Alt_L"),
        "ralt" | "altright" | "altgr" => Some("Alt_R"),
        "super" | "win" | "lwin" | "winleft" | "meta" | "cmd" => Some("Super_L"),
        "rwin" | "winright" => Some("Super_R"),
        _ => None,
    }
}

const UPPER: [&str; 26] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];
const LOWER: [&str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];
const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

/// The keysym that types one character, and whether Shift must be held.
///
/// WHY: `type_text` must send every character exactly, capitals and `>`
/// included. On a US layout the shifted punctuation shares a keycode with
/// its unshifted partner, and `XKeysymToKeycode` finds that base keycode even
/// when given the shifted keysym — so the shifted keysym is named here AND
/// Shift is pressed around it. A character with no keysym is `None`, so the
/// caller says it cannot be typed instead of typing something else.
pub fn character_stroke(character: char) -> Option<(&'static str, bool)> {
    match character {
        '\n' | '\r' => return Some(("Return", false)),
        '\t' => return Some(("Tab", false)),
        ' ' => return Some(("space", false)),
        'A'..='Z' => return Some((UPPER[(character as usize) - ('A' as usize)], true)),
        'a'..='z' => return Some((LOWER[(character as usize) - ('a' as usize)], false)),
        '0'..='9' => return Some((DIGITS[(character as usize) - ('0' as usize)], false)),
        _ => {}
    }
    Some(match character {
        '-' => ("minus", false),
        '_' => ("underscore", true),
        '=' => ("equal", false),
        '+' => ("plus", true),
        '[' => ("bracketleft", false),
        '{' => ("braceleft", true),
        ']' => ("bracketright", false),
        '}' => ("braceright", true),
        ';' => ("semicolon", false),
        ':' => ("colon", true),
        '\'' => ("apostrophe", false),
        '"' => ("quotedbl", true),
        '`' => ("grave", false),
        '~' => ("asciitilde", true),
        '\\' => ("backslash", false),
        '|' => ("bar", true),
        ',' => ("comma", false),
        '<' => ("less", true),
        '.' => ("period", false),
        '>' => ("greater", true),
        '/' => ("slash", false),
        '?' => ("question", true),
        '!' => ("exclam", true),
        '@' => ("at", true),
        '#' => ("numbersign", true),
        '$' => ("dollar", true),
        '%' => ("percent", true),
        '^' => ("asciicircum", true),
        '&' => ("ampersand", true),
        '*' => ("asterisk", true),
        '(' => ("parenleft", true),
        ')' => ("parenright", true),
        _ => return None,
    })
}

/// Whether a character is an uppercase letter or shifted punctuation.
///
/// WHY: it is the decision `type_text` makes about holding Shift, kept pure
/// so the unit test can state the rule without a display.
pub fn is_shifted_character(character: char) -> bool {
    character_stroke(character)
        .map(|(_, shifted)| shifted)
        .unwrap_or(false)
}

/// The XTEST button number for an AHK button name.
///
/// WHY: XTEST numbers buttons 1..5, while AHK writes `Left`, `RButton`,
/// `XButton1` and so on. An unknown button is `None`, never button 1 by
/// accident — a wrong button is a click in the wrong place.
pub fn button_number(button: &str) -> Option<u32> {
    match button.to_ascii_lowercase().as_str() {
        "left" | "lbutton" | "1" => Some(1),
        "middle" | "mbutton" | "2" => Some(2),
        "right" | "rbutton" | "3" => Some(3),
        "xbutton1" | "4" => Some(4),
        "xbutton2" | "5" => Some(5),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// The desktop.
// ---------------------------------------------------------------------------

/// `ZPixmap`: pixels stored as packed integers, the format `XGetImage` needs.
const ZPIXMAP: c_int = 2;
/// Every bit-plane of the drawable.
const ALL_PLANES: c_ulong = c_ulong::MAX;
/// `CurrentTime`: let the X server use its own clock for the focus change.
const CURRENT_TIME: c_ulong = 0;
/// `RevertToParent`: give the focus back when the focused window goes away.
const REVERT_TO_PARENT: c_int = 2;
/// `KeyPress`, the event type whose keysym `pending_keys` reports.
const KEY_PRESS: c_int = 2;
/// Xlib's `XEvent` is a union of 24 `long`s on LP64.
const EVENT_WORDS: usize = 24;
/// Xlib's `XWindowAttributes` is 136 bytes on LP64; a larger buffer is safe.
const ATTRS_BYTES: usize = 256;

/// The X11 desktop, implementing `Keyboard`, `Pointer`, `Screen` and `Windows`.
///
/// WHY one struct for both capabilities: `desktop.rs` implements the traits
/// on the one `LabDesktop`, and a single `Display*` is shared by XTEST keys,
/// XTEST buttons and XGetImage. Opening it lazily (and caching the failure)
/// means a missing library or a missing display is a clear `Err` from every
/// method, never a panic at construction. `X11Keyboard` and `X11Pointer` are
/// aliases of this type so a caller may hold either capability by name.
pub struct X11Desktop {
    /// The open `Display*`, or null until the first method is called.
    display: *mut c_void,
    /// A cached reason for a failed open, so the error is stable and clear.
    failure: Option<String>,
    /// The rectangle the pointer is confined to, when it is confined.
    ///
    /// The confinement is the clamp of the coordinates the pointer
    /// sends, the way KeySharp clamps the events of the mouse: the X
    /// server is never grabbed, and nothing of the session is taken.
    clip: Option<(i64, i64, i64, i64)>,
}

/// The keyboard half of [`X11Desktop`], by name.
///
/// WHY: the rest of the yoko stack asks for a `Keyboard` explicitly; the
/// alias makes that request readable without introducing a second connection.
pub type X11Keyboard = X11Desktop;
/// The pointer half of [`X11Desktop`], by name.
///
/// WHY: the same connection drives keys and buttons, but a caller that only
/// wants the pointer should be able to say so.
pub type X11Pointer = X11Desktop;

impl Default for X11Desktop {
    fn default() -> Self {
        Self::new()
    }
}

impl X11Desktop {
    /// A desktop that has not opened a display yet.
    ///
    /// WHY: constructing must never fail, so a caller on a machine without
    /// X11 can still build the value and get an honest error on first use.
    pub fn new() -> Self {
        Self {
            display: std::ptr::null_mut(),
            failure: None,
            clip: None,
        }
    }

    /// Open the display now, and say why it could not be opened.
    ///
    /// WHY: a caller that wants to fail early (rather than at the first
    /// statement of a script) can ask for the connection up front.
    pub fn open() -> Result<Self, String> {
        let mut desktop = Self::new();
        desktop.connection()?;
        Ok(desktop)
    }

    /// The open `Display*`, opened on first use and reused afterwards.
    fn connection(&mut self) -> Result<*mut c_void, String> {
        if let Some(reason) = &self.failure {
            return Err(reason.clone());
        }
        if !self.display.is_null() {
            return Ok(self.display);
        }
        if !ffi::available() {
            let reason = "libX11.so.6 / libXtst.so.6 or one of their symbols is missing: this \
                          desktop cannot drive X11"
                .to_string();
            self.failure = Some(reason.clone());
            return Err(reason);
        }
        let name = display_name();
        let c_name = CString::new(name.as_str())
            .map_err(|_| format!("the display name {name:?} is not a C string"))?;
        let display = ffi::XOpenDisplay(c_name.as_ptr());
        if display.is_null() {
            let reason = format!(
                "XOpenDisplay({name:?}) failed: no X server at that display (set YOKO_DISPLAY or DISPLAY)"
            );
            self.failure = Some(reason.clone());
            return Err(reason);
        }
        self.display = display;
        Ok(display)
    }

    /// The keycode of an X11 keysym name, with a reason for every failure.
    fn keycode(&self, display: *mut c_void, keysym: &str) -> Result<c_uchar, String> {
        let name = CString::new(keysym).map_err(|_| format!("{keysym:?} is not a C string"))?;
        let sym = ffi::XStringToKeysym(name.as_ptr());
        if sym == 0 {
            return Err(format!("X11 has no keysym named {keysym:?}"));
        }
        let code = ffi::XKeysymToKeycode(display, sym);
        if code == 0 {
            return Err(format!(
                "the X server has no keycode for the keysym {keysym:?} (it is not on this keyboard)"
            ));
        }
        Ok(code)
    }

    /// The keycode for an AHK key or modifier name.
    fn named_keycode(&self, display: *mut c_void, name: &str) -> Result<c_uchar, String> {
        let keysym = modifier_keysym(name)
            .or_else(|| keysym_name(name))
            .ok_or_else(|| format!("unknown X11 key name: {name:?}"))?;
        self.keycode(display, keysym)
    }

    /// The RGB bytes of a rectangle of the root window.
    ///
    /// WHY: `capture`, `pixel` and `pixel_search` all read the same XImage;
    /// keeping the read here means the image is always released, even when
    /// the pixel layout cannot be decoded.
    fn image_rgb(&mut self, x: i64, y: i64, width: i64, height: i64) -> Result<Vec<u8>, String> {
        let (x, y, w, h) = rectangle(x, y, width, height)?;
        let display = self.connection()?;
        let root = ffi::XDefaultRootWindow(display);
        let image = ffi::XGetImage(display, root, x, y, w, h, ALL_PLANES, ZPIXMAP);
        if image.is_null() {
            return Err(format!(
                "XGetImage({x}, {y}, {w}x{h}) failed on the root window"
            ));
        }
        let mut rgb = Vec::with_capacity((w as usize) * (h as usize) * 3);
        let mut problem = None;
        for row in 0..h {
            for column in 0..w {
                match ffi::ximage_pixel(image, column as c_int, row as c_int) {
                    Some((red, green, blue)) => rgb.extend_from_slice(&[red, green, blue]),
                    None => {
                        problem = Some(format!(
                            "the XImage at ({column}, {row}) has a pixel layout this backend cannot read"
                        ));
                        break;
                    }
                }
            }
            if problem.is_some() {
                break;
            }
        }
        ffi::ximage_free(image);
        match problem {
            Some(reason) => Err(reason),
            None => Ok(rgb),
        }
    }

    /// The children of the root, and the id of the active one.
    fn windows(&mut self) -> Result<(Vec<c_ulong>, c_ulong), String> {
        let display = self.connection()?;
        let root = ffi::XDefaultRootWindow(display);
        let children = ffi::query_tree(display, root);
        let active = ffi::active_window(display, root).unwrap_or(0);
        Ok((children, active))
    }

    /// The titles of every child of the root, with each window's id.
    fn titles(&mut self) -> Result<Vec<(c_ulong, String)>, String> {
        let display = self.connection()?;
        let (children, _) = self.windows()?;
        let mut out = Vec::new();
        for window in children {
            if let Some(title) = ffi::fetch_name(display, window) {
                out.push((window, title));
            }
        }
        Ok(out)
    }

    /// Key events the connection has already received, as X11 keysym names.
    ///
    /// WHY: `XPending`/`XNextEvent`/`XLookupString`/`XKeysymToString` are the
    /// read side of the X11 keyboard. A connection that has not selected
    /// `KeyPressMask` on a window has no such events, so the honest answer is
    /// an empty list; the runtime uses this to drain hotkeys, not to claim
    /// which keys are currently held (which XTEST cannot tell — see
    /// `Keyboard::key_is_down`).
    pub fn pending_keys(&mut self) -> Result<Vec<String>, String> {
        let display = self.connection()?;
        let mut names = Vec::new();
        while ffi::XPending(display) > 0 {
            let mut event = [0usize; EVENT_WORDS];
            ffi::XNextEvent(display, event.as_mut_ptr().cast::<c_void>());
            if (event[0] as c_int) != KEY_PRESS {
                continue;
            }
            let mut buffer: [c_char; 32] = [0; 32];
            let mut keysym: c_ulong = 0;
            ffi::XLookupString(
                event.as_ptr().cast::<c_void>(),
                buffer.as_mut_ptr(),
                buffer.len() as c_int,
                &mut keysym,
                std::ptr::null_mut(),
            );
            if let Some(name) = ffi::keysym_to_string(keysym) {
                names.push(name);
            }
        }
        Ok(names)
    }

    /// Raise and focus the first window whose title contains `title`.
    ///
    /// WHY: `XRaiseWindow` and `XSetInputFocus` are the X11 answer to AHK's
    /// `WinActivate`; the uinput backend cannot do this because it never sees
    /// the window manager's stacking order.
    pub fn activate(&mut self, title: &str) -> Result<String, String> {
        if title.is_empty() {
            return Err("the title to activate is empty".to_string());
        }
        let display = self.connection()?;
        let mut found = None;
        for (window, name) in self.titles()? {
            if name.contains(title) {
                found = Some((window, name));
                break;
            }
        }
        let Some((window, name)) = found else {
            return Err(format!("no X11 window has a title containing {title:?}"));
        };
        ffi::XRaiseWindow(display, window);
        ffi::XSetInputFocus(display, window, REVERT_TO_PARENT, CURRENT_TIME);
        ffi::XFlush(display);
        Ok(format!("raised and focused the X11 window {name:?}"))
    }
}

impl Drop for X11Desktop {
    fn drop(&mut self) {
        if !self.display.is_null() {
            ffi::XCloseDisplay(self.display);
            self.display = std::ptr::null_mut();
        }
    }
}

/// Validate a rectangle and turn it into Xlib's `int`/`unsigned int` shape.
///
/// WHY: a negative origin or a size that does not fit an `int` would be
/// silently truncated by the C cast and capture the wrong part of the screen.
fn rectangle(
    x: i64,
    y: i64,
    width: i64,
    height: i64,
) -> Result<(c_int, c_int, c_uint, c_uint), String> {
    if x < 0 || y < 0 || width <= 0 || height <= 0 {
        return Err(format!(
            "the region ({x}, {y}, {width}x{height}) is not a rectangle on the screen"
        ));
    }
    if x > c_int::MAX as i64 || y > c_int::MAX as i64 {
        return Err("the region starts outside the X11 coordinate range".to_string());
    }
    if width > c_int::MAX as i64 || height > c_int::MAX as i64 {
        return Err("the region is bigger than XGetImage can capture".to_string());
    }
    Ok((x as c_int, y as c_int, width as c_uint, height as c_uint))
}

/// Validate a screen position before handing it to `XTestFakeMotionEvent`.
fn position(x: i64, y: i64) -> Result<(c_int, c_int), String> {
    if x < c_int::MIN as i64
        || x > c_int::MAX as i64
        || y < c_int::MIN as i64
        || y > c_int::MAX as i64
    {
        return Err(format!(
            "the position ({x}, {y}) is outside the X11 coordinate range"
        ));
    }
    Ok((x as c_int, y as c_int))
}

/// Whether a title matches, with the same rule `desktop.rs` uses.
fn title_matches(title: &str, wanted: &str, mode: TitleMatch) -> Result<bool, String> {
    if wanted.is_empty() {
        return Ok(false);
    }
    mode.matches(title, wanted)
}

/// A scratch file name for a capture the caller did not name.
fn millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

impl Keyboard for X11Desktop {
    fn type_text(&mut self, text: &str) -> Result<String, String> {
        if text.is_empty() {
            return Ok(String::new());
        }
        let display = self.connection()?;
        let shift = self.keycode(display, "Shift_L")?;
        let mut typed = 0usize;
        for character in text.chars() {
            let (keysym, shifted) = character_stroke(character)
                .ok_or_else(|| format!("the X11 keyboard has no keysym for {character:?}"))?;
            let code = self.keycode(display, keysym)?;
            if shifted {
                ffi::XTestFakeKeyEvent(display, shift as c_uint, 1, 0);
            }
            ffi::XTestFakeKeyEvent(display, code as c_uint, 1, 0);
            ffi::XTestFakeKeyEvent(display, code as c_uint, 0, 0);
            if shifted {
                ffi::XTestFakeKeyEvent(display, shift as c_uint, 0, 0);
            }
            ffi::XFlush(display);
            typed += 1;
        }
        Ok(format!(
            "typed {typed} characters through XTEST on {}",
            display_name()
        ))
    }

    fn press_key(&mut self, name: &str, modifiers: &[String]) -> Result<String, String> {
        let display = self.connection()?;
        let main = self.named_keycode(display, name)?;
        let mut held = Vec::new();
        for modifier in modifiers {
            let code = self.named_keycode(display, modifier)?;
            ffi::XTestFakeKeyEvent(display, code as c_uint, 1, 0);
            held.push(code);
        }
        ffi::XFlush(display);
        ffi::XTestFakeKeyEvent(display, main as c_uint, 1, 0);
        ffi::XTestFakeKeyEvent(display, main as c_uint, 0, 0);
        for code in held.iter().rev() {
            ffi::XTestFakeKeyEvent(display, *code as c_uint, 0, 0);
        }
        ffi::XFlush(display);
        let spelling = if modifiers.is_empty() {
            name.to_string()
        } else {
            format!("{}+{name}", modifiers.join("+"))
        };
        Ok(format!("pressed {spelling} through XTEST"))
    }

    fn key_down(&mut self, name: &str) -> Result<String, String> {
        let display = self.connection()?;
        let code = self.named_keycode(display, name)?;
        ffi::XTestFakeKeyEvent(display, code as c_uint, 1, 0);
        ffi::XFlush(display);
        Ok(format!("{name} is down (keycode {code})"))
    }

    fn key_up(&mut self, name: &str) -> Result<String, String> {
        let display = self.connection()?;
        let code = self.named_keycode(display, name)?;
        ffi::XTestFakeKeyEvent(display, code as c_uint, 0, 0);
        ffi::XFlush(display);
        Ok(format!("{name} is up (keycode {code})"))
    }

    fn key_is_down(&mut self, _name: &str) -> Result<bool, String> {
        Err(
            "this desktop cannot tell which keys are down: XTEST injects key \
             events but exposes no keyboard state (XQueryKeymap is not resolved). \
             Use PixelGetColor or MouseGetPos with PixelGetColor to check the focus."
                .to_string(),
        )
    }

    fn keys_down(&mut self) -> Result<Vec<String>, String> {
        Err(
            "this desktop cannot tell which keys are down: XTEST injects key \
             events but exposes no keyboard state"
                .to_string(),
        )
    }

    fn lock_is_on(&mut self, lock: &str) -> Result<bool, String> {
        // The state of a lock key is the state of the LED of the keyboard,
        // whatever the display server is: the kernel publishes it, and the
        // server writes it.
        crate::locks::is_on(lock)
    }

    fn set_lock(&mut self, lock: &str, on: bool) -> Result<(), String> {
        let now = crate::locks::is_on(lock)?;
        if now == on {
            return Ok(());
        }
        self.press_key(lock, &[])?;
        // The server answers the key at the pace of its own frame, and the
        // LED is what says that it did.
        for _ in 0..50 {
            std::thread::sleep(std::time::Duration::from_millis(20));
            if crate::locks::is_on(lock)? == on {
                return Ok(());
            }
        }
        Err(format!(
            "the X server did not change the state of the {lock}: the key was sent, and the LED of \
             the key is still the one it was."
        ))
    }
}

impl Pointer for X11Desktop {
    fn clip_cursor(&mut self, rect: Option<(i64, i64, i64, i64)>) -> Result<(), String> {
        // The rectangle the script asked for, the far edges exclusive,
        // and nothing of the session is taken: the clamp of the
        // coordinates the pointer sends is the confinement, and the X
        // server is never grabbed.
        self.clip = rect;
        Ok(())
    }

    fn click(&mut self, x: i64, y: i64, button: &str, count: u32) -> Result<String, String> {
        let (x, y) = self.the_clamped(x, y)?;
        let number =
            button_number(button).ok_or_else(|| format!("unknown X11 mouse button: {button:?}"))?;
        let display = self.connection()?;
        let times = count.max(1);
        // XTEST's motion takes a screen number; -1 means the current screen,
        // which is the one the pointer is on.
        ffi::XTestFakeMotionEvent(display, -1, x, y, 0);
        for index in 0..times {
            ffi::XTestFakeButtonEvent(display, number, 1, 0);
            ffi::XTestFakeButtonEvent(display, number, 0, 0);
            ffi::XFlush(display);
            if index + 1 < times {
                std::thread::sleep(std::time::Duration::from_millis(60));
            }
        }
        let suffix = if times == 1 {
            String::new()
        } else {
            format!(" {times} times")
        };
        Ok(format!(
            "clicked ({x}, {y}){suffix} with button {number} through XTEST"
        ))
    }

    fn mouse_down(
        &mut self,
        x: Option<i64>,
        y: Option<i64>,
        button: &str,
    ) -> Result<String, String> {
        let display = self.connection()?;
        self.press_button(display, x, y, button, 1)
    }

    fn mouse_up(&mut self, x: Option<i64>, y: Option<i64>, button: &str) -> Result<String, String> {
        let display = self.connection()?;
        self.press_button(display, x, y, button, 0)
    }

    fn move_pointer(&mut self, x: i64, y: i64) -> Result<String, String> {
        let (x, y) = position(x, y)?;
        let display = self.connection()?;
        ffi::XTestFakeMotionEvent(display, -1, x, y, 0);
        ffi::XFlush(display);
        Ok(format!("moved the pointer to ({x}, {y}) through XTEST"))
    }

    fn mouse_position(&mut self) -> Result<(i64, i64), String> {
        Err(
            "this desktop cannot tell where the pointer is: XQueryPointer is not \
             resolved, and XTEST only writes positions"
                .to_string(),
        )
    }
}

impl X11Desktop {
    /// One half of a button press, with an optional move first.
    ///
    /// WHY: `mouse_down` and `mouse_up` share the move-then-button shape of
    /// the uinput backend, so the shape lives once here.
    /// The position of the pointer, clamped to the rectangle of the
    /// confinement when there is one.
    ///
    /// The far edges of the rectangle are exclusive: a movement that
    /// leaves it is made to its edge.
    fn the_clamped(&self, x: i64, y: i64) -> Result<(i32, i32), String> {
        let (x, y) = position(x, y)?;
        // The position of the X server is a number of thirty-two bits and
        // the rectangle of the confinement is a number of sixty-four: the
        // clamp is made in the wide one, and the answer is cut to the
        // narrow one -- and cut inside it, so that a rectangle the
        // script wrote with a number of the sixty-four does not wrap.
        let (x, y) = (i64::from(x), i64::from(y));
        let (x, y) = match self.clip {
            Some((left, top, right, bottom)) => {
                let left = left.min(right);
                let top = top.min(bottom);
                (
                    x.clamp(left, (right - 1).max(left)),
                    y.clamp(top, (bottom - 1).max(top)),
                )
            }
            None => (x, y),
        };
        let narrow = |value: i64| value.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
        Ok((narrow(x), narrow(y)))
    }

    fn press_button(
        &mut self,
        display: *mut c_void,
        x: Option<i64>,
        y: Option<i64>,
        button: &str,
        is_press: c_int,
    ) -> Result<String, String> {
        let number =
            button_number(button).ok_or_else(|| format!("unknown X11 mouse button: {button:?}"))?;
        if let (Some(x), Some(y)) = (x, y) {
            let (x, y) = position(x, y)?;
            ffi::XTestFakeMotionEvent(display, -1, x, y, 0);
        }
        ffi::XTestFakeButtonEvent(display, number, is_press, 0);
        ffi::XFlush(display);
        let state = if is_press == 1 { "down" } else { "up" };
        Ok(format!("the {button} button is {state} through XTEST"))
    }
}

impl Screen for X11Desktop {
    fn capture(
        &mut self,
        x: i64,
        y: i64,
        width: i64,
        height: i64,
        filename: &str,
    ) -> Result<String, String> {
        let (_, _, w, h) = rectangle(x, y, width, height)?;
        let rgb = self.image_rgb(x, y, width, height)?;
        let path = if filename.is_empty() {
            format!("ahk-x11-{}.png", millis())
        } else {
            filename.to_string()
        };
        let mut file = std::fs::File::create(&path)
            .map_err(|error| format!("the capture {path:?} cannot be written: {error}"))?;
        // The reader of the box writes the file too: one implementation of
        // the format, and the same CRC that a capture written here is read
        // with by the other backend.
        let picture = crate::picture::write_rgb(w as usize, h as usize, &rgb)
            .map_err(|reason| format!("the capture {path:?} cannot be written: {reason}"))?;
        std::io::Write::write_all(&mut file, &picture)
            .map_err(|error| format!("the capture {path:?} cannot be written: {error}"))?;
        Ok(path)
    }

    fn pixel(&mut self, x: i64, y: i64) -> Result<String, String> {
        let rgb = self.image_rgb(x, y, 1, 1)?;
        if rgb.len() < 3 {
            return Err("XGetImage answered no pixel".to_string());
        }
        Ok(format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]))
    }

    fn screen_size(&mut self) -> Result<(i64, i64), String> {
        let display = self.connection()?;
        let root = ffi::XDefaultRootWindow(display);
        let mut attrs = [0u8; ATTRS_BYTES];
        if ffi::XGetWindowAttributes(display, root, attrs.as_mut_ptr().cast::<c_void>()) == 0 {
            return Err("XGetWindowAttributes failed on the root window".to_string());
        }
        // XWindowAttributes starts with `int x, y, width, height;`.
        let width = c_int::from_ne_bytes(attrs[8..12].try_into().expect("four bytes"));
        let height = c_int::from_ne_bytes(attrs[12..16].try_into().expect("four bytes"));
        if width <= 0 || height <= 0 {
            return Err("the X11 root window has no size".to_string());
        }
        Ok((width as i64, height as i64))
    }

    fn pixel_search(
        &mut self,
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        colour: &str,
    ) -> Result<Option<(i64, i64)>, String> {
        let target = parse_colour(colour)?;
        let (left, top) = (x1.min(x2), y1.min(y2));
        let (right, bottom) = (x1.max(x2), y1.max(y2));
        let width = right - left + 1;
        let height = bottom - top + 1;
        if width <= 0 || height <= 0 {
            return Ok(None);
        }
        let rgb = self.image_rgb(left, top, width, height)?;
        for y in 0..height as usize {
            for x in 0..width as usize {
                let at = (y * width as usize + x) * 3;
                if at + 2 < rgb.len()
                    && rgb[at] == target[0]
                    && rgb[at + 1] == target[1]
                    && rgb[at + 2] == target[2]
                {
                    return Ok(Some((left + x as i64, top + y as i64)));
                }
            }
        }
        Ok(None)
    }
}

/// Parse `#rrggbb`, `#rgb` or `rrggbb` into three bytes.
fn parse_colour(colour: &str) -> Result<[u8; 3], String> {
    let text = colour
        .trim()
        .trim_start_matches('#')
        .trim_start_matches("0x");
    let byte = |part: &str| u8::from_str_radix(part, 16).map_err(|error| error.to_string());
    match text.len() {
        6 => Ok([byte(&text[0..2])?, byte(&text[2..4])?, byte(&text[4..6])?]),
        3 => Ok([
            byte(&text[0..1])? * 17,
            byte(&text[1..2])? * 17,
            byte(&text[2..3])? * 17,
        ]),
        _ => Err(format!("{colour:?} is not a colour: write it as #rrggbb")),
    }
}

impl Windows for X11Desktop {
    fn active_window_title(&mut self) -> Result<Option<String>, String> {
        let display = self.connection()?;
        let root = ffi::XDefaultRootWindow(display);
        let Some(id) = ffi::active_window(display, root) else {
            return Ok(None);
        };
        Ok(ffi::fetch_name(display, id))
    }

    fn always_on_top(
        &mut self,
        window: &ahk::desktop::WindowInfo,
    ) -> Result<Option<bool>, String> {
        let display = self.connection()?;
        Ok(ffi::always_on_top(display, window.id as c_ulong))
    }

    fn set_always_on_top(
        &mut self,
        window: &ahk::desktop::WindowInfo,
        on: bool,
    ) -> Result<(), String> {
        let display = self.connection()?;
        let root = ffi::XDefaultRootWindow(display);
        if ffi::set_always_on_top(display, root, window.id as c_ulong, on) {
            Ok(())
        } else {
            Err("the X server refused the client message of the EWMH.".to_string())
        }
    }

    fn window_enabled(
        &mut self,
        window: &ahk::desktop::WindowInfo,
    ) -> Result<Option<bool>, String> {
        let display = self.connection()?;
        Ok(ffi::window_is_enabled(display, window.id as c_ulong))
    }

    fn control_enabled(
        &mut self,
        _window: &ahk::desktop::WindowInfo,
    ) -> Result<Option<bool>, String> {
        // The state of a control is the state of a widget, and it needs the
        // accessibility tree. This backend does not read it.
        Ok(None)
    }

    fn the_window_at(&mut self, x: i64, y: i64) -> Result<Option<i64>, String> {
        // The translation from the root to the root answers the child of
        // the root that holds the point -- the top-level window there --
        // and a child of zero is the root itself, with no window at the
        // point. The identifier is the window of the X server, the same
        // one the enumeration of the windows of this backend carries.
        let display = self.connection()?;
        let root = ffi::XDefaultRootWindow(display);
        let mut dst_x: c_int = 0;
        let mut dst_y: c_int = 0;
        let mut child: c_ulong = 0;
        let translated = ffi::XTranslateCoordinates(
            display,
            root,
            root,
            x as c_int,
            y as c_int,
            &mut dst_x,
            &mut dst_y,
            &mut child,
        );
        if translated == 0 {
            return Err("the X server did not translate the point".to_string());
        }
        Ok(if child == 0 {
            None
        } else {
            Some(child as i64)
        })
    }

    fn window_capabilities(&mut self) -> Vec<String> {

        vec![
            "AlwaysOnTopGet".to_string(),
            "AlwaysOnTopSet".to_string(),
            "WindowEnabledGet".to_string(),
            "WindowList".to_string(),
            "WindowActivate".to_string(),
        ]
    }

    fn window_active(&mut self, title: &str, mode: TitleMatch) -> Result<WindowAnswer, String> {
        if title.is_empty() {
            return Ok(WindowAnswer::Unknown(
                "the title is empty, and v2 would use the last found window: this runtime \
                 does not keep one."
                    .to_string(),
            ));
        }
        let display = self.connection()?;
        let root = ffi::XDefaultRootWindow(display);
        let Some(id) = ffi::active_window(display, root) else {
            return Ok(WindowAnswer::Unknown(
                "the window manager publishes no _NET_ACTIVE_WINDOW; the active window is unknown"
                    .to_string(),
            ));
        };
        let Some(name) = ffi::fetch_name(display, id) else {
            return Ok(WindowAnswer::Unknown(format!(
                "the active X11 window {id} has no WM_NAME"
            )));
        };
        Ok(if title_matches(&name, title, mode)? {
            WindowAnswer::Active
        } else {
            WindowAnswer::NotActive
        })
    }

    fn window_exists(&mut self, title: &str, mode: TitleMatch) -> Result<WindowAnswer, String> {
        if title.is_empty() {
            return Ok(WindowAnswer::Unknown(
                "the title is empty, and v2 would use the last found window: this runtime \
                 does not keep one."
                    .to_string(),
            ));
        }
        let display = self.connection()?;
        let (titles, active) = self.windows()?;
        let mut found = false;
        let mut is_active = false;
        for window in titles {
            let Some(name) = ffi::fetch_name(display, window) else {
                continue;
            };
            if title_matches(&name, title, mode)? {
                found = true;
                if window == active {
                    is_active = true;
                    break;
                }
            }
        }
        if !found {
            Ok(WindowAnswer::Absent)
        } else if is_active {
            Ok(WindowAnswer::Active)
        } else {
            Ok(WindowAnswer::NotActive)
        }
    }
}

// The remaining faces of a `Desktop`. An X11 desktop is a keyboard, a
// pointer, a screen and its windows; the process, file, clipboard and
// timer faces are the box's own, and the trait's defaults say honestly
// what this desktop cannot answer rather than pretending it can. `run` is
// the one method a `Desktop` must have, and it runs on the box itself —
// a script that says `Run` means the machine, not the X server.
impl Processes for X11Desktop {
    fn run(&mut self, command: &str, _window: &str) -> Result<(i32, String, String), String> {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|error| error.to_string())?;
        Ok((
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }

    // The processes of the machine are the same ones whatever the display
    // server is: the reading is the `/proc` one of `crate::process`, and the
    // shape of the answers is the one of the binary, measured on 2026-11-09.

    fn process_exist(&mut self, which: Option<&str>) -> Result<u32, String> {
        crate::process::exist(which)
    }

    fn process_name(&mut self, which: Option<&str>) -> Result<Option<String>, String> {
        crate::process::name(which)
    }

    fn process_path(&mut self, which: Option<&str>) -> Result<Option<String>, String> {
        crate::process::path(which)
    }

    fn process_parent(&mut self, which: Option<&str>) -> Result<u32, String> {
        crate::process::parent(which)
    }

    fn process_close(&mut self, which: &str) -> Result<u32, String> {
        crate::process::close(which)
    }

    fn process_wait(&mut self, which: &str, timeout: Option<f64>) -> Result<u32, String> {
        crate::process::wait(which, timeout)
    }

    fn process_wait_close(&mut self, which: &str, timeout: Option<f64>) -> Result<u32, String> {
        crate::process::wait_close(which, timeout)
    }

    fn process_set_priority(
        &mut self,
        priority: &str,
        which: Option<&str>,
    ) -> Result<Option<u32>, String> {
        crate::process::set_priority(priority, which)
    }
}
impl Files for X11Desktop {
    fn recycle(&mut self, pattern: &str) -> Result<(), String> {
        crate::trash::recycle(pattern)
    }
    fn empty_trash(&mut self, _drive: Option<&str>) -> Result<(), String> {
        crate::trash::empty()
    }
}
impl Clipboard for X11Desktop {}
impl Timers for X11Desktop {}
impl Hotkeys for X11Desktop {}
impl Native for X11Desktop {}
impl Sound for X11Desktop {}
impl Machine for X11Desktop {
    fn ip_addresses(&mut self) -> Result<Vec<String>, String> {
        crate::network::ip_addresses()
    }
    fn output_debug(&mut self, text: &str) -> Result<(), String> {
        // The debugger of the machine is the standard error of the process,
        // and the report of the runner is the standard output.
        eprintln!("{text}");
        Ok(())
    }
    fn drive(&mut self, path: &str) -> Result<Option<ahk::desktop::Drive>, String> {
        crate::drives::drive(path)
    }
    fn drive_type(&mut self, path: &str) -> Result<Option<String>, String> {
        crate::drives::drive_type(path)
    }
    fn drive_status(&mut self, path: &str) -> Result<Option<String>, String> {
        crate::drives::drive_status(path)
    }
}
impl Dialogs for X11Desktop {}
impl Network for X11Desktop {
    fn download(&mut self, url: &str, file: &str) -> Result<String, String> {
        // The curl of the box does not care which session is up.
        crate::network::download(url, file)
    }

    fn send_mail(&mut self, message: &ahk::desktop::MailMessage) -> Result<(), String> {
        crate::network::send_mail(message)
    }
}

impl Pictures for X11Desktop {}
impl Desktop for X11Desktop {}
#[cfg(test)]
mod tests {
    use super::*;

    /// At least twelve AHK names map to the X11 spelling XStringToKeysym
    /// accepts, and the single-character names keep their case.
    #[test]
    fn key_names_map_to_x11_keysyms() {
        let cases = [
            ("Return", "Return"),
            ("Enter", "Return"),
            ("Escape", "Escape"),
            ("Esc", "Escape"),
            ("Tab", "Tab"),
            ("BackSpace", "BackSpace"),
            ("Space", "space"),
            ("Up", "Up"),
            ("Down", "Down"),
            ("Left", "Left"),
            ("Right", "Right"),
            ("Home", "Home"),
            ("End", "End"),
            ("Page_Up", "Prior"),
            ("PgUp", "Prior"),
            ("PgDn", "Next"),
            ("F1", "F1"),
            ("F12", "F12"),
            ("a", "a"),
            ("A", "A"),
            ("Z", "Z"),
            ("0", "0"),
            ("9", "9"),
            (">", "greater"),
            ("?", "question"),
        ];
        assert!(cases.len() >= 12);
        for (name, expected) in cases {
            assert_eq!(keysym_name(name), Some(expected), "{name}");
        }
        assert_eq!(keysym_name("NoSuchKey"), None);
    }

    /// Capitals and shifted punctuation need Shift; plain ones do not.
    #[test]
    fn shifted_characters_are_detected() {
        for character in ['A', 'Z', '>', '<', '?', '!', '"', ':', '_', '~', '{', '|'] {
            assert!(is_shifted_character(character), "{character}");
        }
        for character in ['a', 'z', '0', '9', ' ', '-', '.', '/', '\n', '\t'] {
            assert!(!is_shifted_character(character), "{character}");
        }
        assert_eq!(character_stroke('>'), Some(("greater", true)));
        assert_eq!(character_stroke('A'), Some(("A", true)));
        assert_eq!(character_stroke('a'), Some(("a", false)));
        assert_eq!(character_stroke('ç'), None);
    }

    /// The four modifiers the runtime hands over have left/right keysyms.
    #[test]
    fn modifiers_have_keysyms() {
        assert_eq!(modifier_keysym("ctrl"), Some("Control_L"));
        assert_eq!(modifier_keysym("RCtrl"), Some("Control_R"));
        assert_eq!(modifier_keysym("shift"), Some("Shift_L"));
        assert_eq!(modifier_keysym("alt"), Some("Alt_L"));
        assert_eq!(modifier_keysym("super"), Some("Super_L"));
        assert_eq!(modifier_keysym("nope"), None);
    }

    /// Every symbol name is a NUL-terminated C string and none is repeated.
    /// Pure data: this never calls `dlopen`.
    #[test]
    fn symbol_table_is_well_formed() {
        assert_eq!(ffi::SYMBOLS.len(), ffi::SYMBOL_COUNT);
        const { assert!(ffi::LIBXTST_FROM < ffi::SYMBOL_COUNT) };
        for name in ffi::SYMBOLS {
            assert_eq!(name.last(), Some(&0), "a symbol name must end in NUL");
            assert!(!name[..name.len() - 1].contains(&0), "no interior NUL");
        }
        for (i, name) in ffi::SYMBOLS.iter().enumerate() {
            for (j, other) in ffi::SYMBOLS.iter().enumerate().skip(i + 1) {
                assert_ne!(name, other, "duplicate symbol at {i} and {j}");
            }
        }
        // The two macros are NOT symbols and must not appear in the table.
        for forbidden in [b"XDestroyImage\0".as_slice(), b"XGetPixel\0".as_slice()] {
            assert!(!ffi::SYMBOLS.contains(&forbidden));
        }
    }

    /// The documented LP64 XImage offsets are the ones Xlib's header uses.
    #[test]
    fn ximage_pixel_layout_is_sane() {
        let layout = ximage_pixel_layout();
        assert_eq!(layout.width, 0);
        assert_eq!(layout.height, 4);
        assert_eq!(layout.data, 16);
        assert!(layout.depth < layout.bytes_per_line);
        assert!(layout.bytes_per_line < layout.bpp);
        assert!(layout.bpp < layout.red_mask);
        assert!(layout.red_mask < layout.green_mask);
        assert!(layout.green_mask < layout.blue_mask);
        // The exact numbers are LP64's; a 64-bit Windows pointer is LLP64.
        #[cfg(all(
            target_pointer_width = "64",
            any(target_os = "linux", target_os = "freebsd")
        ))]
        {
            assert_eq!(layout.depth, 40);
            assert_eq!(layout.bytes_per_line, 44);
            assert_eq!(layout.bpp, 48);
            assert_eq!(layout.red_mask, 56);
            assert_eq!(layout.blue_mask, 72);
        }
    }

    /// Mouse buttons and colours are parsed without a display.
    #[test]
    fn button_names_and_colours_are_pure() {
        assert_eq!(button_number("Left"), Some(1));
        assert_eq!(button_number("RButton"), Some(3));
        assert_eq!(button_number("wheel"), None);
        assert_eq!(parse_colour("#ff8000").unwrap(), [255, 128, 0]);
        assert_eq!(parse_colour("abc").unwrap(), [170, 187, 204]);
        assert!(parse_colour("nope").is_err());
    }

    /// A lazy desktop answers an honest error rather than panicking when the
    /// libraries or the display are absent. This never opens a display: the
    /// test builds the value and checks the shape of the error only when the
    /// libraries are genuinely missing on this host.
    #[test]
    fn a_desktop_can_be_built_without_a_display() {
        let mut desktop = X11Desktop::new();
        assert!(desktop.display.is_null());
        assert!(desktop.failure.is_none());
        // `keys_down` is an honest refusal, never a lie about the state.
        assert!(desktop.keys_down().is_err());
        assert!(desktop.key_is_down("a").is_err());
    }
}
