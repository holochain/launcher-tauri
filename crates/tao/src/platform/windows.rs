// Copyright 2014-2021 The winit contributors
// Copyright 2021-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

#![cfg(target_os = "windows")]

use std::path::Path;

pub use crate::platform_impl::hit_test;
use crate::{
  dpi::PhysicalSize,
  event::DeviceId,
  event_loop::EventLoop,
  monitor::MonitorHandle,
  platform_impl::{EventLoop as WindowsEventLoop, Parent, WinIcon},
  window::{BadIcon, Icon, Theme, Window, WindowBuilder},
};
use libc;
use windows::Win32::{
  Foundation::HWND,
  UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};

/// Additional methods on `EventLoop` that are specific to Windows.
pub trait EventLoopExtWindows {
  /// Creates an event loop off of the main thread.
  ///
  /// # `Window` caveats
  ///
  /// Note that any `Window` created on the new thread will be destroyed when the thread
  /// terminates. Attempting to use a `Window` after its parent thread terminates has
  /// unspecified, although explicitly not undefined, behavior.
  fn new_any_thread() -> Self
  where
    Self: Sized;

  /// By default, tao on Windows will attempt to enable process-wide DPI awareness. If that's
  /// undesirable, you can create an `EventLoop` using this function instead.
  fn new_dpi_unaware() -> Self
  where
    Self: Sized;

  /// Creates a DPI-unaware event loop off of the main thread.
  ///
  /// The `Window` caveats in [`new_any_thread`](EventLoopExtWindows::new_any_thread) also apply here.
  fn new_dpi_unaware_any_thread() -> Self
  where
    Self: Sized;
}

impl<T> EventLoopExtWindows for EventLoop<T> {
  #[inline]
  fn new_any_thread() -> Self {
    EventLoop {
      event_loop: WindowsEventLoop::new_any_thread(),
      _marker: ::std::marker::PhantomData,
    }
  }

  #[inline]
  fn new_dpi_unaware() -> Self {
    EventLoop {
      event_loop: WindowsEventLoop::new_dpi_unaware(),
      _marker: ::std::marker::PhantomData,
    }
  }

  #[inline]
  fn new_dpi_unaware_any_thread() -> Self {
    EventLoop {
      event_loop: WindowsEventLoop::new_dpi_unaware_any_thread(),
      _marker: ::std::marker::PhantomData,
    }
  }
}

/// Additional methods on `Window` that are specific to Windows.
pub trait WindowExtWindows {
  /// Returns the HINSTANCE of the window
  fn hinstance(&self) -> *mut libc::c_void;
  /// Returns the native handle that is used by this window.
  ///
  /// The pointer will become invalid when the native window was destroyed.
  fn hwnd(&self) -> *mut libc::c_void;

  /// Enables or disables mouse and keyboard input to the specified window.
  ///
  /// A window must be enabled before it can be activated.
  /// If an application has create a modal dialog box by disabling its owner window
  /// (as described in [`WindowBuilderExtWindows::with_owner_window`]), the application must enable
  /// the owner window before destroying the dialog box.
  /// Otherwise, another window will receive the keyboard focus and be activated.
  ///
  /// If a child window is disabled, it is ignored when the system tries to determine which
  /// window should receive mouse messages.
  ///
  /// For more information, see <https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablewindow#remarks>
  /// and <https://docs.microsoft.com/en-us/windows/win32/winmsg/window-features#disabled-windows>
  fn set_enable(&self, enabled: bool);

  /// This sets `ICON_BIG`. A good ceiling here is 256x256.
  fn set_taskbar_icon(&self, taskbar_icon: Option<Icon>);

  /// Returns the current window theme.
  fn theme(&self) -> Theme;

  /// Reset the dead key state of the keyboard.
  ///
  /// This is useful when a dead key is bound to trigger an action. Then
  /// this function can be called to reset the dead key state so that
  /// follow-up text input won't be affected by the dead key.
  fn reset_dead_keys(&self);

  /// Starts the resizing drag from given edge
  fn begin_resize_drag(&self, edge: isize, button: u32, x: i32, y: i32);

  /// Whether to show the window icon in the taskbar or not.
  fn set_skip_taskbar(&self, skip: bool);
}

impl WindowExtWindows for Window {
  #[inline]
  fn hinstance(&self) -> *mut libc::c_void {
    self.window.hinstance().0 as _
  }

  #[inline]
  fn hwnd(&self) -> *mut libc::c_void {
    self.window.hwnd().0 as _
  }

  #[inline]
  fn set_enable(&self, enabled: bool) {
    unsafe {
      EnableWindow(self.window.hwnd(), enabled);
    }
  }

  #[inline]
  fn set_taskbar_icon(&self, taskbar_icon: Option<Icon>) {
    self.window.set_taskbar_icon(taskbar_icon)
  }

  #[inline]
  fn theme(&self) -> Theme {
    self.window.theme()
  }

  #[inline]
  fn reset_dead_keys(&self) {
    self.window.reset_dead_keys();
  }

  #[inline]
  fn begin_resize_drag(&self, edge: isize, button: u32, x: i32, y: i32) {
    self.window.begin_resize_drag(edge, button, x, y)
  }

  #[inline]
  fn set_skip_taskbar(&self, skip: bool) {
    self.window.set_skip_taskbar(skip);
  }
}

/// Additional methods on `WindowBuilder` that are specific to Windows.
pub trait WindowBuilderExtWindows {
  /// Sets a parent to the window to be created.
  ///
  /// A child window has the WS_CHILD style and is confined to the client area of its parent window.
  ///
  /// For more information, see <https://docs.microsoft.com/en-us/windows/win32/winmsg/window-features#child-windows>
  fn with_parent_window(self, parent: HWND) -> WindowBuilder;

  /// Set an owner to the window to be created. Can be used to create a dialog box, for example.
  /// Can be used in combination with [`WindowExtWindows::set_enable(false)`](WindowExtWindows::set_enable)
  /// on the owner window to create a modal dialog box.
  ///
  /// From MSDN:
  /// - An owned window is always above its owner in the z-order.
  /// - The system automatically destroys an owned window when its owner is destroyed.
  /// - An owned window is hidden when its owner is minimized.
  ///
  /// For more information, see <https://docs.microsoft.com/en-us/windows/win32/winmsg/window-features#owned-windows>
  fn with_owner_window(self, parent: HWND) -> WindowBuilder;

  /// Sets a menu on the window to be created.
  ///
  /// Parent and menu are mutually exclusive; a child window cannot have a menu!
  ///
  /// The menu must have been manually created beforehand with [`windows::Win32::UI::WindowsAndMessaging::CreateMenu`]
  /// or similar.
  ///
  /// Note: Dark mode cannot be supported for win32 menus, it's simply not possible to change how the menus look.
  /// If you use this, it is recommended that you combine it with `with_theme(Some(Theme::Light))` to avoid a jarring effect.
  fn with_menu(self, menu: HMENU) -> WindowBuilder;

  /// This sets `ICON_BIG`. A good ceiling here is 256x256.
  fn with_taskbar_icon(self, taskbar_icon: Option<Icon>) -> WindowBuilder;

  /// This sets `WS_EX_NOREDIRECTIONBITMAP`.
  fn with_no_redirection_bitmap(self, flag: bool) -> WindowBuilder;

  /// Enables or disables drag and drop support (enabled by default). Will interfere with other crates
  /// that use multi-threaded COM API (`CoInitializeEx` with `COINIT_MULTITHREADED` instead of
  /// `COINIT_APARTMENTTHREADED`) on the same thread. Note that tao may still attempt to initialize
  /// COM API regardless of this option. Currently only fullscreen mode does that, but there may be more in the future.
  /// If you need COM API with `COINIT_MULTITHREADED` you must initialize it before calling any tao functions.
  /// See <https://docs.microsoft.com/en-us/windows/win32/api/objbase/nf-objbase-coinitialize#remarks> for more information.
  fn with_drag_and_drop(self, flag: bool) -> WindowBuilder;

  /// Whether to create the window icon with the taskbar icon or not.
  fn with_skip_taskbar(self, skip: bool) -> WindowBuilder;
}

impl WindowBuilderExtWindows for WindowBuilder {
  #[inline]
  fn with_parent_window(mut self, parent: HWND) -> WindowBuilder {
    self.platform_specific.parent = Parent::ChildOf(parent);
    self
  }

  #[inline]
  fn with_owner_window(mut self, parent: HWND) -> WindowBuilder {
    self.platform_specific.parent = Parent::OwnedBy(parent);
    self
  }

  #[inline]
  fn with_menu(mut self, menu: HMENU) -> WindowBuilder {
    self.platform_specific.menu = Some(menu);
    self
  }

  #[inline]
  fn with_taskbar_icon(mut self, taskbar_icon: Option<Icon>) -> WindowBuilder {
    self.platform_specific.taskbar_icon = taskbar_icon;
    self
  }

  #[inline]
  fn with_no_redirection_bitmap(mut self, flag: bool) -> WindowBuilder {
    self.platform_specific.no_redirection_bitmap = flag;
    self
  }

  #[inline]
  fn with_drag_and_drop(mut self, flag: bool) -> WindowBuilder {
    self.platform_specific.drag_and_drop = flag;
    self
  }

  #[inline]
  fn with_skip_taskbar(mut self, skip: bool) -> WindowBuilder {
    self.platform_specific.skip_taskbar = skip;
    self
  }
}

/// Additional methods on `MonitorHandle` that are specific to Windows.
pub trait MonitorHandleExtWindows {
  /// Returns the name of the monitor adapter specific to the Win32 API.
  fn native_id(&self) -> String;

  /// Returns the handle of the monitor - `HMONITOR`.
  fn hmonitor(&self) -> *mut libc::c_void;
}

impl MonitorHandleExtWindows for MonitorHandle {
  #[inline]
  fn native_id(&self) -> String {
    self.inner.native_identifier()
  }

  #[inline]
  fn hmonitor(&self) -> *mut libc::c_void {
    self.inner.hmonitor().0 as _
  }
}

/// Additional methods on `DeviceId` that are specific to Windows.
pub trait DeviceIdExtWindows {
  /// Returns an identifier that persistently refers to this specific device.
  ///
  /// Will return `None` if the device is no longer available.
  fn persistent_identifier(&self) -> Option<String>;
}

impl DeviceIdExtWindows for DeviceId {
  #[inline]
  fn persistent_identifier(&self) -> Option<String> {
    self.0.persistent_identifier()
  }
}

/// Additional methods on `Icon` that are specific to Windows.
pub trait IconExtWindows: Sized {
  /// Create an icon from a file path.
  ///
  /// Specify `size` to load a specific icon size from the file, or `None` to load the default
  /// icon size from the file.
  ///
  /// In cases where the specified size does not exist in the file, Windows may perform scaling
  /// to get an icon of the desired size.
  fn from_path<P: AsRef<Path>>(path: P, size: Option<PhysicalSize<u32>>) -> Result<Self, BadIcon>;

  /// Create an icon from a resource embedded in this executable or library.
  ///
  /// Specify `size` to load a specific icon size from the file, or `None` to load the default
  /// icon size from the file.
  ///
  /// In cases where the specified size does not exist in the file, Windows may perform scaling
  /// to get an icon of the desired size.
  fn from_resource(ordinal: u16, size: Option<PhysicalSize<u32>>) -> Result<Self, BadIcon>;
}

impl IconExtWindows for Icon {
  fn from_path<P: AsRef<Path>>(path: P, size: Option<PhysicalSize<u32>>) -> Result<Self, BadIcon> {
    let win_icon = WinIcon::from_path(path, size)?;
    Ok(Icon { inner: win_icon })
  }

  fn from_resource(ordinal: u16, size: Option<PhysicalSize<u32>>) -> Result<Self, BadIcon> {
    let win_icon = WinIcon::from_resource(ordinal, size)?;
    Ok(Icon { inner: win_icon })
  }
}
