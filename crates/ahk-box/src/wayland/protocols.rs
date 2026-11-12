//! The protocols of COSMIC that this crate speaks.
//!
//! The definitions are the XML files of the protocols, in `protocols/`, and
//! the code here is generated from them by the wayland-scanner. Those files
//! carry the licence of their authors -- MIT -- which the Apache 2.0 of
//! this tree accepts.
//!
//! The listing of the windows is not here: that one is portable, and the
//! `wayland-protocols` carries it. What is here is the extension of COSMIC
//! that says which window is the active one, and the workspaces it needs
//! to answer.

pub mod client {
    use wayland_client;
    use wayland_client::protocol::*;
    // The handles of the listing, which the extension of COSMIC takes as
    // the window it is asked about.
    use wayland_protocols::ext::foreign_toplevel_list::v1::client::*;
    // The workspaces, which the extension of COSMIC reports.
    use wayland_protocols::ext::workspace::v1::client::*;

    // The interfaces of the workspaces. They are in a module of their own
    // because the generator writes a helper of its own in every module
    // it is called in, and two of them in the same module do not agree.
    pub mod __interfaces_workspace {
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("protocols/cosmic-workspace-unstable-v1.xml");
    }

    // The interfaces of the information of the windows. They refer to the
    // workspaces and to the listing, and the statics of those have to be
    // in scope.
    pub mod __interfaces {
        use super::__interfaces_workspace::*;
        use wayland_client::protocol::__interfaces::*;
        use wayland_protocols::ext::foreign_toplevel_list::v1::client::__interfaces::*;
        use wayland_protocols::ext::workspace::v1::client::__interfaces::*;
        wayland_scanner::generate_interfaces!("protocols/cosmic-toplevel-info-unstable-v1.xml");
    }

    use self::__interfaces::*;
    use self::__interfaces_workspace::*;

    wayland_scanner::generate_client_code!("protocols/cosmic-toplevel-info-unstable-v1.xml");
    wayland_scanner::generate_client_code!("protocols/cosmic-workspace-unstable-v1.xml");
}
