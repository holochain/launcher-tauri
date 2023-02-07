// Copyright 2014-2021 The winit contributors
// Copyright 2021-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

use crate::{
  dpi::{PhysicalPosition, Size},
  icon::Icon,
  keyboard::ModifiersState,
  platform_impl::platform::{event_loop, minimal_ime::MinimalIme, util},
  window::{CursorIcon, Fullscreen, Theme, WindowAttributes},
};
use parking_lot::MutexGuard;
use std::io;
use windows::Win32::{
  Foundation::{HWND, LPARAM, RECT, WPARAM},
  Graphics::Gdi::{InvalidateRgn, HRGN},
  UI::WindowsAndMessaging::*,
};

/// Contains information about states and the window that the callback is going to use.
pub struct WindowState {
  pub mouse: MouseProperties,

  /// Used by `WM_GETMINMAXINFO`.
  pub min_size: Option<Size>,
  pub max_size: Option<Size>,

  pub window_icon: Option<Icon>,
  pub taskbar_icon: Option<Icon>,

  pub saved_window: Option<SavedWindow>,
  pub scale_factor: f64,

  pub skip_taskbar: bool,

  pub modifiers_state: ModifiersState,
  pub fullscreen: Option<Fullscreen>,
  pub current_theme: Theme,
  pub preferred_theme: Option<Theme>,
  pub high_surrogate: Option<u16>,

  pub ime_handler: MinimalIme,

  pub window_flags: WindowFlags,

  // Used by WM_NCACTIVATE, WM_SETFOCUS and WM_KILLFOCUS
  pub is_active: bool,
  pub is_focused: bool,
}

#[derive(Clone)]
pub struct SavedWindow {
  pub placement: WINDOWPLACEMENT,
}

#[derive(Clone)]
pub struct MouseProperties {
  pub cursor: CursorIcon,
  pub capture_count: u32,
  cursor_flags: CursorFlags,
  pub last_position: Option<PhysicalPosition<f64>>,
}

bitflags! {
    pub struct CursorFlags: u8 {
        const GRABBED   = 1 << 0;
        const HIDDEN    = 1 << 1;
        const IN_WINDOW = 1 << 2;
    }
}
bitflags! {
    pub struct WindowFlags: u32 {
        const RESIZABLE        = 1 << 0;
        const DECORATIONS      = 1 << 1;
        const VISIBLE          = 1 << 2;
        const ON_TASKBAR       = 1 << 3;
        const ALWAYS_ON_TOP    = 1 << 4;
        const NO_BACK_BUFFER   = 1 << 5;
        const TRANSPARENT      = 1 << 6;
        const CHILD            = 1 << 7;
        const MAXIMIZED        = 1 << 8;
        const POPUP            = 1 << 9;
        const ALWAYS_ON_BOTTOM = 1 << 10;
        const MINIMIZABLE = 1 << 11;
        const MAXIMIZABLE = 1 << 12;

        const MINIMIZED = 1 << 13;

        const IGNORE_CURSOR_EVENT = 1 << 14;

        const CLOSABLE    = 1 << 15;

        /// Marker flag for fullscreen. Should always match `WindowState::fullscreen`, but is
        /// included here to make masking easier.
        const MARKER_EXCLUSIVE_FULLSCREEN = 1 << 16;
        const MARKER_BORDERLESS_FULLSCREEN = 1 << 17;

        /// The `WM_SIZE` event contains some parameters that can effect the state of `WindowFlags`.
        /// In most cases, it's okay to let those parameters change the state. However, when we're
        /// running the `WindowFlags::apply_diff` function, we *don't* want those parameters to
        /// effect our stored state, because the purpose of `apply_diff` is to update the actual
        /// window's state to match our stored state. This controls whether to accept those changes.
        const MARKER_RETAIN_STATE_ON_SIZE = 1 << 18;

        const MARKER_IN_SIZE_MOVE = 1 << 19;

        const MARKER_DONT_FOCUS = 1 << 20;

         /// When minmizing a maximized Windows, `WM_SIZE` which we use to update the `MAXIMIZED` bit
        /// is fired with `wparam` set to `SIZE_MINIMIZED` and thus the `MAXIMIZED` bit will be unset
        /// and when un-minimizing the window, the `MAXIMIZED` bit will still be unset and later on
        /// in `apply_diff` when `new.to_window_styles()` is called, it will not add `WS_MAXIMIZE` window style.
        const MARKER_WAS_MAXIMIZED = 1 << 21;

        const EXCLUSIVE_FULLSCREEN_OR_MASK = WindowFlags::ALWAYS_ON_TOP.bits;
    }
}

impl WindowState {
  pub fn new(
    attributes: &WindowAttributes,
    taskbar_icon: Option<Icon>,
    scale_factor: f64,
    current_theme: Theme,
    preferred_theme: Option<Theme>,
  ) -> WindowState {
    WindowState {
      mouse: MouseProperties {
        cursor: CursorIcon::default(),
        capture_count: 0,
        cursor_flags: CursorFlags::empty(),
        last_position: None,
      },

      min_size: attributes.min_inner_size,
      max_size: attributes.max_inner_size,

      window_icon: attributes.window_icon.clone(),
      taskbar_icon,

      saved_window: None,
      scale_factor,

      skip_taskbar: false,

      modifiers_state: ModifiersState::default(),
      fullscreen: None,
      current_theme,
      preferred_theme,
      high_surrogate: None,
      ime_handler: MinimalIme::default(),
      window_flags: WindowFlags::empty(),
      is_active: false,
      is_focused: false,
    }
  }

  pub fn window_flags(&self) -> WindowFlags {
    self.window_flags
  }

  pub fn set_window_flags<F>(mut this: MutexGuard<'_, Self>, window: HWND, f: F)
  where
    F: FnOnce(&mut WindowFlags),
  {
    let old_flags = this.window_flags;
    f(&mut this.window_flags);
    let new_flags = this.window_flags;

    drop(this);
    old_flags.apply_diff(window, new_flags);
  }

  pub fn set_window_flags_in_place<F>(&mut self, f: F)
  where
    F: FnOnce(&mut WindowFlags),
  {
    f(&mut self.window_flags);
  }

  pub fn has_active_focus(&self) -> bool {
    self.is_active && self.is_focused
  }

  // Updates is_active and returns whether active-focus state has changed
  pub fn set_active(&mut self, is_active: bool) -> bool {
    let old = self.has_active_focus();
    self.is_active = is_active;
    old != self.has_active_focus()
  }

  // Updates is_focused and returns whether active-focus state has changed
  pub fn set_focused(&mut self, is_focused: bool) -> bool {
    let old = self.has_active_focus();
    self.is_focused = is_focused;
    old != self.has_active_focus()
  }
}

impl MouseProperties {
  pub fn cursor_flags(&self) -> CursorFlags {
    self.cursor_flags
  }

  pub fn set_cursor_flags<F>(&mut self, window: HWND, f: F) -> Result<(), io::Error>
  where
    F: FnOnce(&mut CursorFlags),
  {
    let old_flags = self.cursor_flags;
    f(&mut self.cursor_flags);
    match self.cursor_flags.refresh_os_cursor(window) {
      Ok(()) => (),
      Err(e) => {
        self.cursor_flags = old_flags;
        return Err(e);
      }
    }

    Ok(())
  }
}

impl WindowFlags {
  fn mask(mut self) -> WindowFlags {
    if self.contains(WindowFlags::MARKER_EXCLUSIVE_FULLSCREEN) {
      self |= WindowFlags::EXCLUSIVE_FULLSCREEN_OR_MASK;
    }

    self
  }

  pub fn to_window_styles(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE) {
    let (mut style, mut style_ex) = (Default::default(), Default::default());
    style |= WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_SYSMENU | WS_CAPTION;
    style_ex |= WS_EX_ACCEPTFILES;

    if self.contains(WindowFlags::RESIZABLE) {
      style |= WS_SIZEBOX;
    }
    if self.contains(WindowFlags::RESIZABLE | WindowFlags::MAXIMIZABLE) {
      style |= WS_MAXIMIZEBOX;
    }
    if self.contains(WindowFlags::MINIMIZABLE) {
      style |= WS_MINIMIZEBOX;
    }
    if self.contains(WindowFlags::DECORATIONS) {
      style_ex |= WS_EX_WINDOWEDGE;
    }
    if self.contains(WindowFlags::VISIBLE) {
      style |= WS_VISIBLE;
    }
    if self.contains(WindowFlags::ON_TASKBAR) {
      style_ex |= WS_EX_APPWINDOW;
    }
    if self.contains(WindowFlags::ALWAYS_ON_TOP) {
      style_ex |= WS_EX_TOPMOST;
    }
    if self.contains(WindowFlags::NO_BACK_BUFFER) {
      style_ex |= WS_EX_NOREDIRECTIONBITMAP;
    }
    if self.contains(WindowFlags::CHILD) {
      style |= WS_CHILD; // This is incompatible with WS_POPUP if that gets added eventually.
    }
    if self.contains(WindowFlags::POPUP) {
      style |= WS_POPUP;
    }
    if self.contains(WindowFlags::MINIMIZED) {
      style |= WS_MINIMIZE;
    }
    if self.contains(WindowFlags::MAXIMIZED) {
      style |= WS_MAXIMIZE;
    }
    if self.contains(WindowFlags::IGNORE_CURSOR_EVENT) {
      style_ex |= WS_EX_TRANSPARENT | WS_EX_LAYERED;
    }
    if self.intersects(
      WindowFlags::MARKER_EXCLUSIVE_FULLSCREEN | WindowFlags::MARKER_BORDERLESS_FULLSCREEN,
    ) {
      style &= !WS_OVERLAPPEDWINDOW;
    }

    (style, style_ex)
  }

  /// Adjust the window client rectangle to the return value, if present.
  fn apply_diff(mut self, window: HWND, mut new: WindowFlags) {
    self = self.mask();
    new = new.mask();

    let diff = self ^ new;

    if diff == WindowFlags::empty() {
      return;
    }

    if new.contains(WindowFlags::VISIBLE) {
      unsafe {
        ShowWindow(
          window,
          if self.contains(WindowFlags::MARKER_DONT_FOCUS) {
            self.set(WindowFlags::MARKER_DONT_FOCUS, false);
            SW_SHOWNOACTIVATE
          } else {
            SW_SHOW
          },
        );
      }
    }

    if diff.contains(WindowFlags::ALWAYS_ON_TOP) {
      unsafe {
        SetWindowPos(
          window,
          match new.contains(WindowFlags::ALWAYS_ON_TOP) {
            true => HWND_TOPMOST,
            false => HWND_NOTOPMOST,
          },
          0,
          0,
          0,
          0,
          SWP_ASYNCWINDOWPOS | SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        );
        InvalidateRgn(window, HRGN::default(), false);
      }
    }

    if diff.contains(WindowFlags::ALWAYS_ON_BOTTOM) {
      unsafe {
        SetWindowPos(
          window,
          match new.contains(WindowFlags::ALWAYS_ON_BOTTOM) {
            true => HWND_BOTTOM,
            false => HWND_NOTOPMOST,
          },
          0,
          0,
          0,
          0,
          SWP_ASYNCWINDOWPOS | SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        );
        InvalidateRgn(window, HRGN::default(), false);
      }
    }

    if diff.contains(WindowFlags::MAXIMIZED) || new.contains(WindowFlags::MAXIMIZED) {
      unsafe {
        ShowWindow(
          window,
          match new.contains(WindowFlags::MAXIMIZED) {
            true => SW_MAXIMIZE,
            false => SW_RESTORE,
          },
        );
      }
    }

    // Minimize operations should execute after maximize for proper window animations
    if diff.contains(WindowFlags::MINIMIZED) {
      unsafe {
        ShowWindow(
          window,
          match new.contains(WindowFlags::MINIMIZED) {
            true => SW_MINIMIZE,
            false => SW_RESTORE,
          },
        );
      }
    }

    if diff.contains(WindowFlags::CLOSABLE) || new.contains(WindowFlags::CLOSABLE) {
      unsafe {
        let system_menu = GetSystemMenu(window, false);
        EnableMenuItem(
          system_menu,
          SC_CLOSE,
          MF_BYCOMMAND
            | if new.contains(WindowFlags::CLOSABLE) {
              MF_ENABLED
            } else {
              MF_GRAYED
            },
        );
      }
    }

    if !new.contains(WindowFlags::VISIBLE) {
      unsafe {
        ShowWindow(window, SW_HIDE);
      }
    }

    if diff != WindowFlags::empty() {
      let (mut style, style_ex) = new.to_window_styles();

      unsafe {
        SendMessageW(
          window,
          *event_loop::SET_RETAIN_STATE_ON_SIZE_MSG_ID,
          WPARAM(1),
          LPARAM(0),
        );

        // This condition is necessary to avoid having an unrestorable window
        if !new.contains(WindowFlags::MINIMIZED) {
          if self.contains(WindowFlags::MARKER_WAS_MAXIMIZED) {
            style |= WS_MAXIMIZE;
          }

          SetWindowLongW(window, GWL_STYLE, style.0 as i32);
          SetWindowLongW(window, GWL_EXSTYLE, style_ex.0 as i32);
        }

        let mut flags = SWP_NOZORDER | SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED;

        // We generally don't want style changes here to affect window
        // focus, but for fullscreen windows they must be activated
        // (i.e. focused) so that they appear on top of the taskbar
        if !new.contains(WindowFlags::MARKER_EXCLUSIVE_FULLSCREEN)
          && !new.contains(WindowFlags::MARKER_BORDERLESS_FULLSCREEN)
        {
          flags |= SWP_NOACTIVATE;
        }

        // Refresh the window frame
        SetWindowPos(window, HWND::default(), 0, 0, 0, 0, flags);
        SendMessageW(
          window,
          *event_loop::SET_RETAIN_STATE_ON_SIZE_MSG_ID,
          WPARAM(0),
          LPARAM(0),
        );
      }
    }
  }
}

impl CursorFlags {
  fn refresh_os_cursor(self, window: HWND) -> Result<(), io::Error> {
    let client_rect = util::get_client_rect(window)?;

    if util::is_focused(window) {
      let cursor_clip = match self.contains(CursorFlags::GRABBED) {
        true => Some(client_rect),
        false => None,
      };

      let rect_to_tuple = |rect: RECT| (rect.left, rect.top, rect.right, rect.bottom);
      let active_cursor_clip = rect_to_tuple(util::get_cursor_clip()?);
      let desktop_rect = rect_to_tuple(util::get_desktop_rect());

      let active_cursor_clip = match desktop_rect == active_cursor_clip {
        true => None,
        false => Some(active_cursor_clip),
      };

      // We do this check because calling `set_cursor_clip` incessantly will flood the event
      // loop with `WM_MOUSEMOVE` events, and `refresh_os_cursor` is called by `set_cursor_flags`
      // which at times gets called once every iteration of the eventloop.
      if active_cursor_clip != cursor_clip.map(rect_to_tuple) {
        util::set_cursor_clip(cursor_clip)?;
      }
    }

    let cursor_in_client = self.contains(CursorFlags::IN_WINDOW);
    if cursor_in_client {
      util::set_cursor_hidden(self.contains(CursorFlags::HIDDEN));
    } else {
      util::set_cursor_hidden(false);
    }

    Ok(())
  }
}
