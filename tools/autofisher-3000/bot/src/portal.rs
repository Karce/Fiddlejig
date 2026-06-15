//! XDG desktop portal session.
//!
//! One RemoteDesktop session that *also* carries a ScreenCast stream, so the
//! captured frames and the injected pointer/keyboard share a single coordinate
//! space (no capture→screen calibration). GNOME shows a "share screen + allow
//! control" dialog each run — it does **not** let a session that injects input
//! persist its grant ("Remote desktop sessions cannot persist"), so the dialog
//! can't be remembered while the portal is used for input.

use anyhow::{Context, Result};
use ashpd::desktop::remote_desktop::{DeviceType, KeyState, RemoteDesktop, SelectDevicesOptions};
use ashpd::desktop::screencast::{CursorMode, Screencast, SelectSourcesOptions, SourceType};
use ashpd::desktop::{PersistMode, Session};
use ashpd::enumflags2::BitFlags;
use std::os::fd::{AsRawFd, OwnedFd, RawFd};

/// Linux evdev code for the right mouse button (BTN_RIGHT) — reels in the bobber.
const BTN_RIGHT: i32 = 0x111;

/// A live portal session: the RemoteDesktop proxy + session (used for input in
/// `input.rs`) and the ScreenCast stream details (used to build the capture
/// pipeline). Holding the session keeps the stream alive; dropping it ends both.
pub struct PortalSession {
    pub remote_desktop: RemoteDesktop,
    pub session: Session<RemoteDesktop>,
    /// PipeWire node id of the chosen monitor/window stream.
    pub node_id: u32,
    /// Stream logical size reported by the portal.
    pub width: u32,
    pub height: u32,
    /// PipeWire remote fd — must stay open for the capture pipeline.
    pipewire_fd: OwnedFd,
}

impl PortalSession {
    /// Open the linked RemoteDesktop + ScreenCast session. Prompts on first run.
    pub async fn open() -> Result<Self> {
        let remote_desktop = RemoteDesktop::new()
            .await
            .context("connecting to the RemoteDesktop portal")?;
        let screencast = Screencast::new()
            .await
            .context("connecting to the ScreenCast portal")?;

        let session = remote_desktop
            .create_session(Default::default())
            .await
            .context("creating the portal session")?;

        remote_desktop
            .select_devices(
                &session,
                SelectDevicesOptions::default()
                    .set_devices(DeviceType::Keyboard | DeviceType::Pointer),
            )
            .await
            .context("selecting keyboard + pointer devices")?;

        screencast
            .select_sources(
                &session,
                SelectSourcesOptions::default()
                    // keep the cursor out of the frame so it can't be mistaken for a bobber
                    .set_cursor_mode(CursorMode::Hidden)
                    // windows-only so the picker opens straight to the window grid
                    .set_sources(BitFlags::from(SourceType::Window))
                    .set_multiple(false)
                    // GNOME forbids persisting an input-injecting session (see module docs)
                    .set_persist_mode(PersistMode::DoNot),
            )
            .await
            .context("selecting the capture source")?;

        let response = remote_desktop
            .start(&session, None, Default::default())
            .await
            .context("starting the portal session")?
            .response()
            .context("the screen-share / control dialog was cancelled or denied")?;

        let stream = response
            .streams()
            .first()
            .context("portal returned no screencast stream — pick a monitor or window")?;
        let node_id = stream.pipe_wire_node_id();
        let (width, height) = stream.size().unwrap_or((0, 0));

        let pipewire_fd = screencast
            .open_pipe_wire_remote(&session, Default::default())
            .await
            .context("opening the PipeWire remote")?;

        Ok(Self {
            remote_desktop,
            session,
            node_id,
            width: width.max(0) as u32,
            height: height.max(0) as u32,
            pipewire_fd,
        })
    }

    /// Raw PipeWire fd for the GStreamer `pipewiresrc fd=…` element.
    pub fn pipewire_fd(&self) -> RawFd {
        self.pipewire_fd.as_raw_fd()
    }

    // --- input injection (frame coordinates == stream coordinates) ---

    /// Move the pointer to a frame coordinate within the captured stream.
    pub async fn move_to(&self, x: f64, y: f64) -> Result<()> {
        self.remote_desktop
            .notify_pointer_motion_absolute(&self.session, self.node_id, x, y, Default::default())
            .await
            .context("notify_pointer_motion_absolute")?;
        Ok(())
    }

    /// Right-click at the current pointer position.
    pub async fn right_click(&self) -> Result<()> {
        self.remote_desktop
            .notify_pointer_button(
                &self.session,
                BTN_RIGHT,
                KeyState::Pressed,
                Default::default(),
            )
            .await
            .context("notify_pointer_button press")?;
        self.remote_desktop
            .notify_pointer_button(
                &self.session,
                BTN_RIGHT,
                KeyState::Released,
                Default::default(),
            )
            .await
            .context("notify_pointer_button release")?;
        Ok(())
    }

    /// Press and release a key, given as a Linux evdev keycode.
    pub async fn press_key(&self, keycode: i32) -> Result<()> {
        self.remote_desktop
            .notify_keyboard_keycode(
                &self.session,
                keycode,
                KeyState::Pressed,
                Default::default(),
            )
            .await
            .context("notify_keyboard_keycode press")?;
        self.remote_desktop
            .notify_keyboard_keycode(
                &self.session,
                keycode,
                KeyState::Released,
                Default::default(),
            )
            .await
            .context("notify_keyboard_keycode release")?;
        Ok(())
    }
}
