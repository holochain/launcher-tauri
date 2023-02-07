# Changelog

## \[0.15.8]

- Add `with_cursor_moved` Unix extension trait method.
  - [8c6b2d05](https://github.com/tauri-apps/tao/commit/8c6b2d05ae55cefe72bd63d1adfeca6c20058879) Add `with_cursor_moved` unix extension method ([#644](https://github.com/tauri-apps/tao/pull/644)) on 2022-12-14

## \[0.15.7]

- On Linux, fix menu item mnemonics.
  - [86a439ed](https://github.com/tauri-apps/tao/commit/86a439edc5da2bd1baa1067831dde2408fd14fbf) fix: fix menu mnemonics ([#640](https://github.com/tauri-apps/tao/pull/640)) on 2022-12-08
  - [e623efdc](https://github.com/tauri-apps/tao/commit/e623efdc9ab797b3d9e104f34ba5bc1a4648b32c) publish new versions ([#639](https://github.com/tauri-apps/tao/pull/639)) on 2022-12-10
  - [bdce0a4c](https://github.com/tauri-apps/tao/commit/bdce0a4c816bb63b9e52114924ee4a66d353a019) Revert "publish new versions ([#639](https://github.com/tauri-apps/tao/pull/639))" on 2022-12-10
- On Windows, retain `WS_MAXIMIZE` window style when unminimizing a maximized window.
  - [ca844a2e](https://github.com/tauri-apps/tao/commit/ca844a2ebb171f676962bd0bebd65243b9239347) fix(Windows): retain `WS_MAXIMIZE` when unminimizing a maximized window, closes [#622](https://github.com/tauri-apps/tao/pull/622) ([#638](https://github.com/tauri-apps/tao/pull/638)) on 2022-12-04
  - [e623efdc](https://github.com/tauri-apps/tao/commit/e623efdc9ab797b3d9e104f34ba5bc1a4648b32c) publish new versions ([#639](https://github.com/tauri-apps/tao/pull/639)) on 2022-12-10
  - [bdce0a4c](https://github.com/tauri-apps/tao/commit/bdce0a4c816bb63b9e52114924ee4a66d353a019) Revert "publish new versions ([#639](https://github.com/tauri-apps/tao/pull/639))" on 2022-12-10
- On macOS, strip menu mnemonics for consistency with other platforms.
  - [86a439ed](https://github.com/tauri-apps/tao/commit/86a439edc5da2bd1baa1067831dde2408fd14fbf) fix: fix menu mnemonics ([#640](https://github.com/tauri-apps/tao/pull/640)) on 2022-12-08
  - [e623efdc](https://github.com/tauri-apps/tao/commit/e623efdc9ab797b3d9e104f34ba5bc1a4648b32c) publish new versions ([#639](https://github.com/tauri-apps/tao/pull/639)) on 2022-12-10
  - [bdce0a4c](https://github.com/tauri-apps/tao/commit/bdce0a4c816bb63b9e52114924ee4a66d353a019) Revert "publish new versions ([#639](https://github.com/tauri-apps/tao/pull/639))" on 2022-12-10

## \[0.15.6]

- Revert `nextResponder` call because this will bring key beep sound regression. We'll call the key equivalent in wry instead.
  - [a59b69b2](https://github.com/tauri-apps/tao/commit/a59b69b2733b273d86dc200ba90065be3db871a6) On macOS, revert nextResponder calls ([#628](https://github.com/tauri-apps/tao/pull/628)) on 2022-11-21

## \[0.15.5]

- Change `WebviewAttributes::focused` default to `true`.
  - [ece3e8f6](https://github.com/tauri-apps/tao/commit/ece3e8f6a34de21ec4c19944f668edd47ecc8ce0) fix: default `focused` to true on 2022-11-20
- On Linux, wake the main context in `EventLoopProxy::send_event()`.
  - [b7b5f04d](https://github.com/tauri-apps/tao/commit/b7b5f04d4b4c2f58146aca1b7e03223cdae74f7c) Gtk: wake the main context in EventLoopProxy::send_event(), closes [#625](https://github.com/tauri-apps/tao/pull/625) ([#626](https://github.com/tauri-apps/tao/pull/626)) on 2022-11-16

## \[0.15.4]

- On macOS, call next responder in view's keyDown and doCommandbySelector.
  - [516e5fcd](https://github.com/tauri-apps/tao/commit/516e5fcd50de601330f3434ecd00bf5889f1a5cc) On macOS, remove `doCommandBySelector` in view ([#620](https://github.com/tauri-apps/tao/pull/620)) on 2022-11-09
  - [e9d6dadb](https://github.com/tauri-apps/tao/commit/e9d6dadb59fd8d5d32704a5d80d8d587f5d581ca) Publish New Versions ([#621](https://github.com/tauri-apps/tao/pull/621)) on 2022-11-09
  - [045b768e](https://github.com/tauri-apps/tao/commit/045b768e30b4dc261edcaba4b8ed8ec9fee8305e) On macOS, call next responder in view's keyDown and doCommandbySelector ([#623](https://github.com/tauri-apps/tao/pull/623)) on 2022-11-14

## \[0.15.3]

- On macOS, remove `doCommandBySelector` in view since this will block the key event to responder chain.
  - [516e5fcd](https://github.com/tauri-apps/tao/commit/516e5fcd50de601330f3434ecd00bf5889f1a5cc) On macOS, remove `doCommandBySelector` in view ([#620](https://github.com/tauri-apps/tao/pull/620)) on 2022-11-09

## \[0.15.2]

- On Windows, fix compliation regression introduced in 0.15.1 when `tray` feature is active
  - [081664dc](https://github.com/tauri-apps/tao/commit/081664dc6b12c7765b667072dfbfbc089e50c5a3) fix(Windows): fix build regression when tray feature is used ([#618](https://github.com/tauri-apps/tao/pull/618)) on 2022-11-09

## \[0.15.1]

- On Windows, fix window always visible initially.
  - [ae06c3e2](https://github.com/tauri-apps/tao/commit/ae06c3e2806b85a9baa10b84c898cd0c15af7de4) fix(Windows): fix windows always visible initially on 2022-11-08

## \[0.15.0]

- Add support for parsing `ArrowUp`, `ArrowDown`, `ArrowLeft` and `ArrowRight` in a str as valid key. Previously only `Up`, `Down`, `Left` and `Right` worked.
  - [5e85dbef](https://github.com/tauri-apps/tao/commit/5e85dbef325fd8050b5cc26079627a3b645285c7) fix: parse `Arrow*` in a accelerator string ([#609](https://github.com/tauri-apps/tao/pull/609)) on 2022-10-31
- Add `WindowBuilder::with_content_protection`.
  - [8084c800](https://github.com/tauri-apps/tao/commit/8084c8001c2b8aa00abe7305765c19ce4e8ffc66) feat: add `WindowBuilder::with_content_protection` ([#605](https://github.com/tauri-apps/tao/pull/605)) on 2022-10-30
- On macOS, fix default cursor always being arrow cursor
  - [1359fccf](https://github.com/tauri-apps/tao/commit/1359fccfe5b95de9a28afb92e9ac3adfc331fb3c) On macOS, fix default cursor always being arrow cursor ([#614](https://github.com/tauri-apps/tao/pull/614)) on 2022-11-06
- On Windows, fixed focus event emission on minimize.
  - [37bca310](https://github.com/tauri-apps/tao/commit/37bca310be977f8922eb13e35d0e53b925a6039d) fix(windows): fix focus event emission on minimize ([#559](https://github.com/tauri-apps/tao/pull/559)) on 2022-09-21
- Update jni to 0.20.
  - [38fef108](https://github.com/tauri-apps/tao/commit/38fef1087d217874aee016e237b6c15eedbd0250) feat(android): update to jni 0.20 ([#610](https://github.com/tauri-apps/tao/pull/610)) on 2022-10-31
- On Linux, add DeviceEvent::Key.
  - [775974d7](https://github.com/tauri-apps/tao/commit/775974d7084185b80afe76c5acf3727687b0fd02) feat(linux): add DeviceEvent::Key ([#600](https://github.com/tauri-apps/tao/pull/600)) on 2022-10-21
- fix(linux): Improve event loop process on Linux a bit. This changes only a few check and should make dragging windows on egui smoother.
  - [b529eec9](https://github.com/tauri-apps/tao/commit/b529eec9ba32d8d3195e1866bdb26b93beba7e13) fix(linux): improve event loop process on Linux ([#587](https://github.com/tauri-apps/tao/pull/587)) on 2022-10-12
- Fix inverted delta in `WindowEvent::MouseWheel` on Linux
  - [8451f754](https://github.com/tauri-apps/tao/commit/8451f754a97ddfc5f9cb3d81ed3ea4ae38c173f3) fix: Inverse mouse scroll wheel on Linux ([#585](https://github.com/tauri-apps/tao/pull/585)) on 2022-10-11
- Add `EventLoopExtMacOS::set_activate_ignoring_other_apps` on macOS.
  - [d2c6a91c](https://github.com/tauri-apps/tao/commit/d2c6a91c4588bab460ef5924b10cfb224c1336c6) feat: add `EventLoopExtMacOS::set_activate_ignoring_other_apps` ([#612](https://github.com/tauri-apps/tao/pull/612)) on 2022-11-01
- Add `WindowExtMacOS::set_allows_automatic_window_tabbing`, `WindowExtMacOS::allows_automatic_window_tabbing`, and `WindowBuilderExtMacOS::with_automatic_window_tabbing` on macOS.
  - [7c7ce8ab](https://github.com/tauri-apps/tao/commit/7c7ce8ab2d838a79ecdf83df00124c418a6a51f6) feat(macos): add `allows_automatic_window_tabbing` APIs ([#586](https://github.com/tauri-apps/tao/pull/586)) on 2022-10-12
- Support cross compiling for macos from a non macos host.
  - [2edc7418](https://github.com/tauri-apps/tao/commit/2edc74182378c09eddae22de9fb77a301f2cbbf4) Fix cross compilation. ([#601](https://github.com/tauri-apps/tao/pull/601)) on 2022-10-25
- Add `WindowExtMacOS::is_doucmented_edited` and `WindowExtMacOS::set_is_doucmented_edited` on macOS.
  - [33fdeab6](https://github.com/tauri-apps/tao/commit/33fdeab6291d4aef8ea9facb58fe9583f6c1aaf3) feat(macos): add document edited apis, closes [#268](https://github.com/tauri-apps/tao/pull/268) ([#287](https://github.com/tauri-apps/tao/pull/287)) on 2022-10-03
- On macOS, scale menu item icons height to 18.
  - [5e3d344c](https://github.com/tauri-apps/tao/commit/5e3d344c77006fce03459a851e30cb798e568756) fix(macos): scale menu item icon height to 18, closes [#584](https://github.com/tauri-apps/tao/pull/584) ([#590](https://github.com/tauri-apps/tao/pull/590)) on 2022-10-15
- Add support for the "+" key in menu accelerators using `KeyCode::Plus` or the "Plus" keyword.
  See documentation for `KeyCode::Plus` for notes on platform-dependent behaviour.
  - [937aba7b](https://github.com/tauri-apps/tao/commit/937aba7b7faba04e8d154f7681f97985f0b8ca76) feat(menus): add support for Plus key in accelerators, closes [#227](https://github.com/tauri-apps/tao/pull/227) ([#573](https://github.com/tauri-apps/tao/pull/573)) on 2022-09-27
- Add the application name to the "Quit" and "Hide" native menu items on macOS.
  - [65f768e5](https://github.com/tauri-apps/tao/commit/65f768e55fb1eb53642246c63194ef75e84f908a) fix(menus): add app name to native Quit and Hide items on macOS, closes [#536](https://github.com/tauri-apps/tao/pull/536) ([#570](https://github.com/tauri-apps/tao/pull/570)) on 2022-09-25
- Fix the native Services menu on macOS.
  - [d343abf8](https://github.com/tauri-apps/tao/commit/d343abf8ccc67dff8bc6db2b370a652683360f64) fix(menus): fix macOS Services menu not working, closes [#243](https://github.com/tauri-apps/tao/pull/243) ([#569](https://github.com/tauri-apps/tao/pull/569)) on 2022-09-25
- Scale the tray icon according to its aspect ratio on macOS.
  - [dbbfd97c](https://github.com/tauri-apps/tao/commit/dbbfd97c615ba0eec582b484e06608b87f34ef7a) feat(macos): support to change tray icon aspect ratio, close [#564](https://github.com/tauri-apps/tao/pull/564) ([#565](https://github.com/tauri-apps/tao/pull/565)) on 2022-09-25
- Add builder methods on Linux to control the drawing behavior of the window. `WindowBuilderExtUnix::with_double_buffered`, `WindowBuilderExtUnix::with_rgba_visual` and `WindowBuilderExtUnix::with_app_paintable`
  - [0637c605](https://github.com/tauri-apps/tao/commit/0637c605bd74eaf6ac9995a340bcc650a46664e8) feat(linux): add drawing behavior builder methods, closes [#567](https://github.com/tauri-apps/tao/pull/567) ([#572](https://github.com/tauri-apps/tao/pull/572)) on 2022-09-27
- On Windows, show Window menu (also known as the System menu or Control menu) in response to <kbd>Alt+Space</kbd>.
  - [0d76094e](https://github.com/tauri-apps/tao/commit/0d76094e90252285002a63cb4d889c6c8c485bdf) fix(Windows): show window menu on alt+space, closes 547 ([#593](https://github.com/tauri-apps/tao/pull/593)) on 2022-10-19
- On Windows, fix icons specified on `WindowBuilder` not taking effect for windows created after the firt one.
  - [d72b1e1a](https://github.com/tauri-apps/tao/commit/d72b1e1a0cb7757bdc67008d70c8d25596ad393d) fix(Windows): fix icons specified on `WindowBuilder` not taking effect for windows created after the first one ([#604](https://github.com/tauri-apps/tao/pull/604)) on 2022-10-27
- Added tabbing identifier APIs on macOS.
  - [8815291e](https://github.com/tauri-apps/tao/commit/8815291e8cf4ac855ae4b9ad93183a27d6da5bb7) feat(macos): add tabbing identifier APIs ([#592](https://github.com/tauri-apps/tao/pull/592)) on 2022-10-18
- On Linux, reduce channel redirect. Now sending user events and redraw request will send to event loops directly.
  - [dd86a9eb](https://github.com/tauri-apps/tao/commit/dd86a9ebec67bf103e79bdfd1a2377cbe832bc03) refactor(linux): reduce channel redirect ([#588](https://github.com/tauri-apps/tao/pull/588)) on 2022-10-16
- Add `WindowBuilder::with_focused` to specify whether to initially focus the window or not.
  - [e42ff071](https://github.com/tauri-apps/tao/commit/e42ff07190c37b6b8a7808900133838247a730df) feat: add `WindowBuilder::with_focused` ([#576](https://github.com/tauri-apps/tao/pull/576)) on 2022-10-03
- Add APIs for disabling the individual window controls on desktop platforms, `Window::set_closable`, `Window::is_closable`, `WindowBuilder::with_closable`, `Window::set_minimizable`, `Window::is_minimizable`, `WindowBuilder::with_minimizable`, `Window::set_maximizable`, `Window::is_maximizable`, `WindowBuilder::with_maximizable`. See the docs for platform-specific notes, especially regarding Linux.
  - [a50fd867](https://github.com/tauri-apps/tao/commit/a50fd867b3df033dff077f5320b4b7037b04e454) feat: options to disable individual window controls, closes [#116](https://github.com/tauri-apps/tao/pull/116) ([#574](https://github.com/tauri-apps/tao/pull/574)) on 2022-10-11
- Add `Window::title` to get the current window title.
  - [c50529b3](https://github.com/tauri-apps/tao/commit/c50529b3ee453c73acf36bd7e1cd7c14669951f3) feat: add `Window::title` getter, closes [#546](https://github.com/tauri-apps/tao/pull/546) ([#579](https://github.com/tauri-apps/tao/pull/579)) on 2022-10-04
- Default to MOD_NOREPEAT for registering global shortcuts / hotkeys via win32 RegisterHotKey on Windows. This prevents shortcuts from repeatedly activating when the accelerator is pressed and held down, and ensures that we maintain platform-agnostic consistency.
  - [d15a756c](https://github.com/tauri-apps/tao/commit/d15a756cfca5d7ec7b4a526edd68c3576f308d9a) Prevent global shortcut activation from repeating on Windows ([#602](https://github.com/tauri-apps/tao/pull/602)) on 2022-10-23

## \[0.14.0]

- Implement "always on bottom" as contrary to "always on top".
  - [a2a7b726](https://github.com/tauri-apps/tao/commit/a2a7b7262cc55e4c6defb79d5f77efce9d7e386d) Always on bottom ([#522](https://github.com/tauri-apps/tao/pull/522)) on 2022-08-22
- Fix calling android functions when package name contained escaped underscore.
  - [6d8cc7e3](https://github.com/tauri-apps/tao/commit/6d8cc7e3e4091462a741ee748112a3ea4aa4f12f) fix(android): unescape escaped underscore in package name ([#531](https://github.com/tauri-apps/tao/pull/531)) on 2022-08-16
- Add `Window::set_content_protection` for macOS and Windows.
  - [802146fb](https://github.com/tauri-apps/tao/commit/802146fb8692a46185846a64163c174520450c43) feat: implement set_content_protection, closes [#550](https://github.com/tauri-apps/tao/pull/550) ([#551](https://github.com/tauri-apps/tao/pull/551)) on 2022-09-04
- - Add DeviceEventFilter on Windows.
- **Breaking**: On Windows, device events are now ignored for unfocused windows by default, use `EventLoopWindowTarget::set_device_event_filter` to set the filter level.
- [5bbd4f8f](https://github.com/tauri-apps/tao/commit/5bbd4f8f72901425432a35915d79d0bee0c96cce) Add DeviceEventFilter on Windows ([#465](https://github.com/tauri-apps/tao/pull/465)) on 2022-08-17
- Fix system tray creation after event loop starts on macOS.
  - [759b7db3](https://github.com/tauri-apps/tao/commit/759b7db37b8188ea38fa2919f9a0e504d4d2edca) fix(macos): retain tray to prevent segfault when event loop is running ([#539](https://github.com/tauri-apps/tao/pull/539)) on 2022-08-20
- Fix resize doesn't work when calling with resizable. Also add platform specific note to `set_resizable`.
  On Linux, most size methods like maximized are async and do not work well with calling
  sequentailly. For setting inner or outer size, you don't need to set resizable to true before
  it. It can resize no matter what. But if you insist to do so, it has a `100, 100` minimum
  limitation somehow. For maximizing, it requires resizable is true. If you really want to set
  resizable to false after it. You might need a mechanism to check the window is really
  maximized.
  - [4524d5d3](https://github.com/tauri-apps/tao/commit/4524d5d399c8bf01d22b160a9f9a04d5b074b466) fix(Linux): resize doesn't work when calling with resizable, fix [#545](https://github.com/tauri-apps/tao/pull/545) ([#553](https://github.com/tauri-apps/tao/pull/553)) on 2022-09-08
- Add `Window::is_focused`.
  - [7d2eeeeb](https://github.com/tauri-apps/tao/commit/7d2eeeebb4da15e9aeda9bf17e80ebdf23c95cee) feat: Window::is_focused ([#533](https://github.com/tauri-apps/tao/pull/533)) on 2022-08-17
- On Linux, fix global shortcut are never triggered when a Lock key is ON, eg. NumLock, CapsLock.
  - [07e3c1f5](https://github.com/tauri-apps/tao/commit/07e3c1f55d18537dc5c776b2706490676bba7cde) fix(linux/globalShorcut): extract needed mods from event state, closes [#307](https://github.com/tauri-apps/tao/pull/307), closes [#537](https://github.com/tauri-apps/tao/pull/537) ([#538](https://github.com/tauri-apps/tao/pull/538)) on 2022-08-19
  - [871ad037](https://github.com/tauri-apps/tao/commit/871ad037b02b8ab4d650ba390664386e195c0bc7) chore: remove changefile, bug still exists on 2022-08-20
  - [7e5556e0](https://github.com/tauri-apps/tao/commit/7e5556e0f247076e8f547fca313d957eeca46366) fix(linux/globalShortcut): grab the shortcut with extra mods, closes [#307](https://github.com/tauri-apps/tao/pull/307) ([#540](https://github.com/tauri-apps/tao/pull/540)) on 2022-08-20
- Disables the global shortcut manager on wayland as its X11-specific.
  - [27ab6f4d](https://github.com/tauri-apps/tao/commit/27ab6f4dcef19c052b0434873e34b54447b70860) fix(linux/globalShortcut): disable on wayland ([#543](https://github.com/tauri-apps/tao/pull/543)) on 2022-08-26
- Added `SystemTrayExtMacOS::set_title` to `SystemTray` and `SystemTrayBuilderExtMacOS::with_title` to set the tray icon title on MacOS
  - [972307dd](https://github.com/tauri-apps/tao/commit/972307ddf088b0f941be2ea66bded2473222aed5) feat: added text support to system tray for macos, closes [#65](https://github.com/tauri-apps/tao/pull/65) ([#554](https://github.com/tauri-apps/tao/pull/554)) on 2022-09-10
- Update `windows-rs` to the latest 0.39.0 release.

The `alloc` feature has been removed, which means it no longer accepts Rust `String` or `&str` parameters and implicitly converts them to `PWSTR` or `PSTR`.

For string literals, that feature was replaced with `s!()` and `w!()` macros which null terminate the string literal at compile time and convert to UTF-16 if necessary. The `s!()` macro is fine, however the `w!()` macro uses `HSTRING` types from WinRT for maximum compatibility with WinRT types. Since Tao only uses Win32 APIs, this change relies on `util::encode_wide` to convert to a `Vec<u16>` instead.

- [84e1a9f9](https://github.com/tauri-apps/tao/commit/84e1a9f93fa7e9e83f1ed92320ab9d7998673c60) Update windows to 0.39.0 ([#544](https://github.com/tauri-apps/tao/pull/544)) on 2022-08-31

## \[0.13.3]

- Implement custom protocol on Android.
  - [b464b8ae](https://github.com/tauri-apps/tao/commit/b464b8ae296cf2b545cd0a98a1506f83779e94ff) feat(android): implement custom protocol ([#527](https://github.com/tauri-apps/tao/pull/527)) on 2022-08-13
- Changed `WebViewMessage::Eval` to evaluate an specific script.
  - [903c7e7f](https://github.com/tauri-apps/tao/commit/903c7e7f5b8c7984515697865fc7c74b496a64dc) feat(android): change WebViewMessage::Eval to run specific script ([#529](https://github.com/tauri-apps/tao/pull/529)) on 2022-08-13
- Fix webview initialization scripts implementation on Android.
  - [3d66ad0b](https://github.com/tauri-apps/tao/commit/3d66ad0b5548ed40da6e954fb5a911c3fb5a13e8) fix(android): run initialization scripts before page loads ([#528](https://github.com/tauri-apps/tao/pull/528)) on 2022-08-13
- Removed the webview logic from the Android glue.
  - [152aaa44](https://github.com/tauri-apps/tao/commit/152aaa4481ff8e44fc32ea3fe93a74c7fecd5be5) refactor(android): remove WebView logic, allow wry to hook into it ([#530](https://github.com/tauri-apps/tao/pull/530)) on 2022-08-14
- Implement `SystemTray::set_tooltip` and `SystemTrayBuilder::with_tooltip` on Windows.
  - [06949a79](https://github.com/tauri-apps/tao/commit/06949a7948100a51e98008c9e6f4ac73e069433a) feat(windows): implement `with_tooltip`&`set_tooltip`, closes [#205](https://github.com/tauri-apps/tao/pull/205) ([#524](https://github.com/tauri-apps/tao/pull/524)) on 2022-08-10

## \[0.13.2]

- Remove the NSStatusItem from the menu bar when the `SystemTray` instance is dropped.
  - [aca4d3fb](https://github.com/tauri-apps/tao/commit/aca4d3fb2619d8bd38e4514583e921227cba6a04) feat(tray): remove from tray on `Drop` on macOS ([#520](https://github.com/tauri-apps/tao/pull/520)) on 2022-08-04
- Fixes `Window::is_decorated` always returning `true` on macOS.
  - [c3e076e9](https://github.com/tauri-apps/tao/commit/c3e076e9345ad33183426f5ef4bd936305254e15) fix(window): `is_decorated` wrong return value, closes [#518](https://github.com/tauri-apps/tao/pull/518) ([#519](https://github.com/tauri-apps/tao/pull/519)) on 2022-08-04
- Fix theme feature to support Darker theme on Linux.
  - [c6d6c011](https://github.com/tauri-apps/tao/commit/c6d6c0115c2facd488e8fab73c8f8b92e172771c) fix: support Darker theme on Linux ([#511](https://github.com/tauri-apps/tao/pull/511)) on 2022-08-03
- Add `Window::is_minimized()`.
  - [9c348154](https://github.com/tauri-apps/tao/commit/9c3481548b05de1d56c2efe8a1951fd014006b27) feat: add `Window::is_minimized()`, closes [#257](https://github.com/tauri-apps/tao/pull/257) ([#486](https://github.com/tauri-apps/tao/pull/486)) on 2022-08-06
- Implement `SystemTrayBuilder::with_tooltip` and `SystemTray::set_tooltip` on macOS.
  - [14e26568](https://github.com/tauri-apps/tao/commit/14e265682fab87502d59a718c9607aaf146c4d3e) feat(macos): add `SystemTray::set_tooltip`, ref [#409](https://github.com/tauri-apps/tao/pull/409) ([#410](https://github.com/tauri-apps/tao/pull/410)) on 2022-08-03
- On Windows, fix a ghost window appearing occasionally when clicking the tray icon.
  - [ad1f641f](https://github.com/tauri-apps/tao/commit/ad1f641f496c21a02c8d173167d77f1b31849273) fix(windows): fix tray event window showing up on click, closes [#506](https://github.com/tauri-apps/tao/pull/506) ([#507](https://github.com/tauri-apps/tao/pull/507)) on 2022-08-02
- Added `SystemTrayBuilder::with_id` and the `id` field to `Event::TrayEvent` for better multitray support.
  - [4ea78bcb](https://github.com/tauri-apps/tao/commit/4ea78bcb577f36ebd4f6b7ce4fcd31d7c02cafdb) feat(tray): add identifier to allow multiple tray setup ([#514](https://github.com/tauri-apps/tao/pull/514)) on 2022-08-04
- Hide the app indicator when dropping `SystemTray` on Linux
  - [9c6a543c](https://github.com/tauri-apps/tao/commit/9c6a543c1a748ed53ae408780a65043f6a9448f9) feat(tray): hide indicator on drop on Linux ([#521](https://github.com/tauri-apps/tao/pull/521)) on 2022-08-04

## \[0.13.1]

- On Linux, fix Window can't be displayed on wayland.
  - [eb880f48](https://github.com/tauri-apps/tao/commit/eb880f48932adb96bc428efdf69e2256fe989b6b) Fix window can't be displayed on wayland ([#504](https://github.com/tauri-apps/tao/pull/504)) on 2022-07-28

## \[0.13.0]

- On Linux, receive only one draw event per cycle to prevent receiving infinite draw events.
  - [b86ada73](https://github.com/tauri-apps/tao/commit/b86ada73ccc493340f4cee35d884867623287111) Receive only one draw event per cycle ([#500](https://github.com/tauri-apps/tao/pull/500)) on 2022-07-25
- - On Linux, add `EventLoopWindowTargetExtUnix` for methods to determine if the backend is x11 or wayland.
- On Linux, add `x11` module for glutin internal use. This is basically just x11-dl, but winit secretly exports it.
- On Linux, add `WindowBuilder::with_transparent_draw` to disable the internal draw for transparent window and allows users to draw it manually.
- [db7e5cb4](https://github.com/tauri-apps/tao/commit/db7e5cb4466133869f512487e605b061a6610560) feat(linux): Add necessary features for creating GL windows ([#495](https://github.com/tauri-apps/tao/pull/495)) on 2022-07-25
- **Breaking** Updated `raw-window-handle` to `0.5` and added `Window::raw_display_handle` and `EventLoopWindowTarget::raw_display_handle`.
  - [b905852d](https://github.com/tauri-apps/tao/commit/b905852d2e76dafaadc8c0ca5785328981628bf0) chore(deps): update `raw-window-handle` to `0.5` ([#493](https://github.com/tauri-apps/tao/pull/493)) on 2022-07-24
- On Windows, respect min/max inner sizes when creating the window.
  - [c1c6822e](https://github.com/tauri-apps/tao/commit/c1c6822e8bb4708857225491b939f076b120dec1) fix(windows): respect min/max sizes when creating window, closes [#498](https://github.com/tauri-apps/tao/pull/498) ([#499](https://github.com/tauri-apps/tao/pull/499)) on 2022-07-25

## \[0.12.2]

- On Windows, fix assigning the wrong mintor rect to undecorated maximized window. This caused a blank window downstream in wry and tauri.
  - [9d97e4a6](https://github.com/tauri-apps/tao/commit/9d97e4a646ce8a4372d3ed2c22d227d8a33ba8ba) fix(windows): get correct monitor in `WM_NCCALCSIZE`, closes [#471](https://github.com/tauri-apps/tao/pull/471) ([#472](https://github.com/tauri-apps/tao/pull/472)) on 2022-07-12
- Fixed set_inner_size is reset when resizable is set to false.
  - [17203d08](https://github.com/tauri-apps/tao/commit/17203d08a4ee49c8fa8decb24bcf76fe4c264ca7) fix: fixed inner_size even if resizable is set to false ([#461](https://github.com/tauri-apps/tao/pull/461)) on 2022-07-05
- On Windows, prevent ghost window from showing up in the taskbar after either several hours of use or restarting `explorer.exe`.
  - [feb21272](https://github.com/tauri-apps/tao/commit/feb212726da553397beda6428666092b0561fd12) fix(windows): prevent ghost window from showing up on taskbar ([#489](https://github.com/tauri-apps/tao/pull/489)) on 2022-07-21
- Add theme feature on Linux.
  - [74425e8e](https://github.com/tauri-apps/tao/commit/74425e8e5b032299cec99f8278d9a05ae650013c) feat: add theme feature on Linux ([#468](https://github.com/tauri-apps/tao/pull/468)) on 2022-07-10
- Fix maximizing window on Linux.
  - [01fb1d6c](https://github.com/tauri-apps/tao/commit/01fb1d6cdf17f01ebe5757f4a66a0d8e40222490) fix: maximizing window on linux, closes [#442](https://github.com/tauri-apps/tao/pull/442) ([#456](https://github.com/tauri-apps/tao/pull/456)) on 2022-07-12
- On macOS, fallback resize event for NSWindow to handle.
  - [ab2e57e9](https://github.com/tauri-apps/tao/commit/ab2e57e9ec056861fa772262a2128c2ac2e16d1b) On macOS, fallback resize event for NSWindow to handle on 2022-07-12
- Add `CustomMenuItem::set_icon`. Only implemented on macOS for now.
  - [13f9f182](https://github.com/tauri-apps/tao/commit/13f9f182754b0bfbaa9163330aed4d444b1e007a) feat(macos): implement CustomMenuItem::set_icon() ([#459](https://github.com/tauri-apps/tao/pull/459)) on 2022-07-07
- On Windows, subscribe to taskbar restart event and re-add the system tray icon.
  Also skip the window from the taskbar if it was already skipped.
  - [9450329e](https://github.com/tauri-apps/tao/commit/9450329e3ab70aa3608ef44207df19cfdddf45a0) fix(windows): subscribe to taskbar restart event, closes [#476](https://github.com/tauri-apps/tao/pull/476) ([#487](https://github.com/tauri-apps/tao/pull/487)) on 2022-07-21
- On Windows, fix focus events being sent to inactive windows.
  - [23ae71b7](https://github.com/tauri-apps/tao/commit/23ae71b717184e2eb0f2da0c683b7c8f0b5cd216) fix(windows): fix focus events being sent to inactive windows. ([#488](https://github.com/tauri-apps/tao/pull/488)) on 2022-07-21

## \[0.12.1]

- Revert #427 due to random crash caused by it.
  - [38f9a587](https://github.com/tauri-apps/tao/commit/38f9a587c5a394a88c767b28e1bf2ae40f990bae) Revert "Remove most RedrawWindow to event target window" ([#457](https://github.com/tauri-apps/tao/pull/457)) on 2022-07-01

## \[0.12.0]

- On macOS, fix native file dialogs hanging the event loop and
  having multiple windows would prevent `run_return` from ever returning.
  - [5c9cc21a](https://github.com/tauri-apps/tao/commit/5c9cc21a394b3d6b32c794453344263457f7d223) Fix native file dialogs freezing the event loop ([#440](https://github.com/tauri-apps/tao/pull/440)) on 2022-06-22
- Fix maximizing window.
- On Windows, fix wrong fullscreen monitors being recognized when handling `WM_WINDOWPOSCHANGING` messages
  - [054a34ec](https://github.com/tauri-apps/tao/commit/054a34ec504dc98235d1fafc9b1cdede7727193e) fix: fix assigning the wrong monitor when receiving Windows move events ([#438](https://github.com/tauri-apps/tao/pull/438)) on 2022-06-22
- Fix global hide others shortcut.
  - [dfae373e](https://github.com/tauri-apps/tao/commit/dfae373e58da44ab6adc977ffe24e3d55ed51de0) fix: global hide others shortcut ([#447](https://github.com/tauri-apps/tao/pull/447)) on 2022-06-25
- Fix window can't be hidden when maximized.
  - [cd9ad33a](https://github.com/tauri-apps/tao/commit/cd9ad33a088b3ab5dbcf3ff1681ebce323c5c61d) Fix window can't be hidden when maximized ([#384](https://github.com/tauri-apps/tao/pull/384)) on 2022-06-15
- On macOS, `WindowEvent::Resized` is now emitted in `frameDidChange` instead of `windowDidResize`.
  - [54062ca1](https://github.com/tauri-apps/tao/commit/54062ca1a96e3637acee771e9657ef0267933352) fix: emit resize event on frame_did_change on macOS, closes [#436](https://github.com/tauri-apps/tao/pull/436) ([#439](https://github.com/tauri-apps/tao/pull/439)) on 2022-06-22
- On Linux, adds `SystemTrayBuilderExtLinux::with_temp_icon_dir` which sets a custom temp icon dir to store generated icon files.
  This may be useful when the application requires icons to be stored in a specific location, such as when running in a Flatpak sandbox.
  - [ce209d39](https://github.com/tauri-apps/tao/commit/ce209d39ab21e493ba98bab83aa7827fa05d7050) feat(linux) add `with_temp_icon_dir` builder extension ([#452](https://github.com/tauri-apps/tao/pull/452)) on 2022-06-26
- On Linux, store tray icons in `$XDG_RUNTIME_DIR`.
  This is preferred over `/tmp`, because this directory (typically `/run/user/{uid}`)
  is only readable for the current user. While `/tmp` is shared with all users.
  - [01253829](https://github.com/tauri-apps/tao/commit/01253829dc23b7316db8941ed8c302d479890186) feat(linux): store tray icons in `XDG_RUNTIME_DIR` ([#449](https://github.com/tauri-apps/tao/pull/449)) on 2022-06-25
- Do not emit the `ThemeChanged` event when the window theme is set and the system theme changes (the window keeps its theme in this scenario).
  - [aae6bec9](https://github.com/tauri-apps/tao/commit/aae6bec9110c19ed0b6618f08e6ce48f483bbfd0) fix(macos): do not emit ThemeChanged event if window theme didn't change ([#430](https://github.com/tauri-apps/tao/pull/430)) on 2022-06-20
- Remvoe `core-video-sys` dependency.
  - [3bb09aa6](https://github.com/tauri-apps/tao/commit/3bb09aa6a03c39bb78378fb85e977c37d8a47a79) fix: remove core-video-sys dependency, closes [#435](https://github.com/tauri-apps/tao/pull/435) ([#441](https://github.com/tauri-apps/tao/pull/441)) on 2022-06-22
- The `theme` function now `Theme::Light` on macOS older than 10.14 and the initial theme setter has no effect instead of crashing the application.
  - [ba9c5571](https://github.com/tauri-apps/tao/commit/ba9c5571f408bdca8584b1b44cc1b95a927d8e34) fix(macos): guard theme APIs to not crash when running on 10.13 or older ([#429](https://github.com/tauri-apps/tao/pull/429)) on 2022-06-20
- Reduce `WM_PAINT` singal on event target window to prevent from webview2 delay.
  - [5ca39af1](https://github.com/tauri-apps/tao/commit/5ca39af1117677b469f92a8094769610c01419ad) Remove most RedrawWindow to event target window ([#427](https://github.com/tauri-apps/tao/pull/427)) on 2022-06-28

## \[0.11.2]

- Fixes the `Ivar menu_on_left_click not found on class TaoTrayHandler` panic on macOS.
  - [2cc163d2](https://github.com/tauri-apps/tao/commit/2cc163d2debba48457a63f4a839f1371b572e121) fix(macos): crash on tray class usage on 2022-06-14

## \[0.11.1]

- Fix macOS `SystemTrayExtMacOS` implementation.
  - [f42c1be1](https://github.com/tauri-apps/tao/commit/f42c1be13ce949a3ca47c9126c2c1e914dee179a) fix: fix wrong macOS trait implementation on 2022-06-14

## \[0.11.0]

- **Breaking change** `SystemTrayBuilder::new` and `SystemTray::set_icon` now takes `system_tray::Icon` on all platforms.
  - [0a98eb39](https://github.com/tauri-apps/tao/commit/0a98eb3993d9f24323f71520426712009bd9e272) refactor: system tray icons ([#328](https://github.com/tauri-apps/tao/pull/328)) on 2022-06-06
- Allow to disable system tray menu only on Left Click.
  - [0858356f](https://github.com/tauri-apps/tao/commit/0858356f3a14fcb6e1e1dfc8d2d35482388ccb43) feat(macos): allow to disable system tray menu on left click, closes [#317](https://github.com/tauri-apps/tao/pull/317) ([#329](https://github.com/tauri-apps/tao/pull/329)) on 2022-06-09
- Connect mouse wheel event with GTK window.
  - [f9e0b734](https://github.com/tauri-apps/tao/commit/f9e0b734c6a3737174d63a0ec8cb2ebc130f35f8) connect mouse wheel event with GTK window ([#412](https://github.com/tauri-apps/tao/pull/412)) on 2022-06-08
- Support child window on Linux.
  - [f1e8d755](https://github.com/tauri-apps/tao/commit/f1e8d7556eb9aea89769a9b38407e8fcd12675af) feat: support child window on linux, closes [#273](https://github.com/tauri-apps/tao/pull/273) ([#415](https://github.com/tauri-apps/tao/pull/415)) on 2022-06-13
- Support theme on macOS.
  - [8af4d8f0](https://github.com/tauri-apps/tao/commit/8af4d8f02149f08093cc348e278f5792dab4a423) feat: support theme on macOS ([#408](https://github.com/tauri-apps/tao/pull/408)) on 2022-06-01
- Add `Window::set_ignore_cursor_events`
  - [4fa87617](https://github.com/tauri-apps/tao/commit/4fa8761776d546ee3b1b0bb1a02a31d72eedfa80) feat: `Window::set_ignore_cursor_events`, closes [#184](https://github.com/tauri-apps/tao/pull/184) ([#421](https://github.com/tauri-apps/tao/pull/421)) on 2022-06-13

## \[0.10.0]

- Fix movable window background on macOS.
  - [e0520b48](https://github.com/tauri-apps/tao/commit/e0520b488bd95167c73c971448e83775032037af) fix: fix movable window background on macOS, closes [#406](https://github.com/tauri-apps/tao/pull/406) ([#405](https://github.com/tauri-apps/tao/pull/405)) on 2022-05-27
- Remove trivial tray features.
  - [f1bd25e6](https://github.com/tauri-apps/tao/commit/f1bd25e643b8a8a656ee64678e9f95135feafb39) Remove trivial tray features ([#411](https://github.com/tauri-apps/tao/pull/411)) on 2022-05-30

## \[0.9.1]

- Fix the size of the slice passed to `DragQueryFileW` by passing `std::mem::transmute(path_buf.spare_capacity_mut())` instead of `&mut path_buf`.
  - [d0dbfa1a](https://github.com/tauri-apps/tao/commit/d0dbfa1a1274f16348701a63b213e9e92776cd74) Fix drag drop on Windows ([#401](https://github.com/tauri-apps/tao/pull/401)) on 2022-05-23

## \[0.9.0]

- Add standalone webview ndk port.
  - [68c9f07e](https://github.com/tauri-apps/tao/commit/68c9f07e8e24690ddf449ebc545ebd90cd29e118) Implement standalone webview ndk ([#385](https://github.com/tauri-apps/tao/pull/385)) on 2022-05-19
- Update the `windows` crate to the latest 0.37.0 release.

The `#[implement]` macro in `windows-implement` and the `implement` feature in `windows` depend on some `const` generic features which stabilized in `rustc` 1.61. The MSRV on Windows targets is effectively 1.61, but other targets do not require these features.

Since developers on non-Windows platforms are not always able to upgrade their toolchain with `rustup`, the package remains at 1.56. Windows developers may get less friendly compiler errors about using unstable language features until they upgrade their toolchain if they build `tao` without `wry`, which has some Windows-specific dependencies that transitively raise the MSRV for `wry` to 1.61.

- [93c256f9](https://github.com/tauri-apps/tao/commit/93c256f9835b2da853129f2a1d77287aa714934e) Update the windows-rs crate to 0.37.0 ([#400](https://github.com/tauri-apps/tao/pull/400)) on 2022-05-23

## \[0.8.5]

- The `current_monitor` function now fallbacks to the primary monitor when the window is invisible.
  - [6cdb99fd](https://github.com/tauri-apps/tao/commit/6cdb99fd271acc6bc2f1132c799c202c444ec7f2) fix(linux): fallback to primary monitor on `current_monitor` impl ([#395](https://github.com/tauri-apps/tao/pull/395)) on 2022-05-18
- Change menubar background color to transparent on Linux when the window is transparent.
  - [a0d9408b](https://github.com/tauri-apps/tao/commit/a0d9408bb89f8dd60687885c2344f7d1069e73d0) fix(linux): make menubar background transparent ([#389](https://github.com/tauri-apps/tao/pull/389)) on 2022-05-14
- Rename full screen menu label to "Toggle Full Screen".
  - [8945f544](https://github.com/tauri-apps/tao/commit/8945f544e77d5cd22a1f3e2ad492c4a2dc986268) fix: rename full screen menu label, closes [#391](https://github.com/tauri-apps/tao/pull/391) ([#393](https://github.com/tauri-apps/tao/pull/393)) on 2022-05-17

## \[0.8.4]

- On Windows, remove the accelerator from `CustomMenuItem::title` returnd string.
  - [634116fe](https://github.com/tauri-apps/tao/commit/634116feb3fe983070fbea79e780caea6d4e7581) fix(Windows): remove accel str from `CustomMenuItem::title` returned string ([#377](https://github.com/tauri-apps/tao/pull/377)) on 2022-04-28
- On Windows and Linux, increase the resizing area for borderless windows based on scale factor.
  - [8701f64a](https://github.com/tauri-apps/tao/commit/8701f64aaec862dc15510ee725498d954521f123) fix: scale borderless resizing inset based on scale_factor, closes [#376](https://github.com/tauri-apps/tao/pull/376) ([#379](https://github.com/tauri-apps/tao/pull/379)) on 2022-05-01

## \[0.8.3]

- Implement `Window::set_cursor_position` for Linux.
  - [afffaeae](https://github.com/tauri-apps/tao/commit/afffaeae665e06804ec1f2a7056afb27431baf10) feat(linux): implement `Window::set_cursor_position` ([#373](https://github.com/tauri-apps/tao/pull/373)) on 2022-04-23

## \[0.8.2]

- Do not fire `WindowEvent::Moved` when `is_maximized` is called on macOS.
  - [25890b94](https://github.com/tauri-apps/tao/commit/25890b943f3566cb8b2fc6d5abaff15921caed93) fix(macos): do not fire Event::Moved when checking is_maximized ([#366](https://github.com/tauri-apps/tao/pull/366)) on 2022-04-13

## \[0.8.1]

- Fixes compilation when only the `tray` feature is enabled.
  - [da938957](https://github.com/tauri-apps/tao/commit/da9389573daa04217baa8465709328e9c6e35f27) fix(tao): compilation when only the tray feature is enabled ([#363](https://github.com/tauri-apps/tao/pull/363)) on 2022-04-05

## \[0.8.0]

- Add `EventLoopWindowTargetExtMacOS::set_activation_policy_at_runtime`.
  - [ef06c508](https://github.com/tauri-apps/tao/commit/ef06c508f1d29e62834eba63b604bf7566b1fef6) Set activation policy at runtime ([#353](https://github.com/tauri-apps/tao/pull/353)) on 2022-03-30
- On Windows and Linux, disable resizing maximized borderless windows.
  - [13c5c996](https://github.com/tauri-apps/tao/commit/13c5c996d15cee9ed829f6e67e786721d8d2eda8) fix(win,linux): disable resizing maximized borderless windows ([#356](https://github.com/tauri-apps/tao/pull/356)) on 2022-03-30
- **Breaking change:** Renamed the `ayatana` Cargo feature to `ayatana-tray`, now the default feature for tray on Linux, and added the `gtk-tray` feature.
  - [40ec796d](https://github.com/tauri-apps/tao/commit/40ec796de4da91640872161ae372124d14777d7f) refactor(tray): split gtk and ayatana appindicator features ([#362](https://github.com/tauri-apps/tao/pull/362)) on 2022-04-05
- - On Windows, Fix random characters when changing menu items title through `CustomMenunItem::set_title`.
  - [e4725bf5](https://github.com/tauri-apps/tao/commit/e4725bf50fb46e830fa5265d765ee596b80e3085) fix(Windows): fix random chars when changing menu item title ([#361](https://github.com/tauri-apps/tao/pull/361)) on 2022-03-31
- On Windows, Fix `Window::set_inner_size` setting a bigger size than requested.
  - [089f3878](https://github.com/tauri-apps/tao/commit/089f3878c5b0ce221d3f405c1215c895ee9fb1ce) fix(Windows): fix `set_inner_size` setting a bigger size, closes [#194](https://github.com/tauri-apps/tao/pull/194) ([#354](https://github.com/tauri-apps/tao/pull/354)) on 2022-04-03

## \[0.7.0]

- Fire `Event::LoopDestroyed` when the macOS dock `Quit` menu item is clicked.
  - [34257a75](https://github.com/tauri-apps/tao/commit/34257a75c71e40433a57942cc61ce9976b80c152) feat(macos): fire `LoopDestroyed` when the dock's `Quit` item is clicked ([#351](https://github.com/tauri-apps/tao/pull/351)) on 2022-03-27
- Added `Event::DecorationsClick` (Windows only).
  - [411af5b1](https://github.com/tauri-apps/tao/commit/411af5b16d71eec90be47210fc6242526ab43c6c) feat(windows): add `Event::DecorationsClick` ([#352](https://github.com/tauri-apps/tao/pull/352)) on 2022-03-27
- Enhance the `MenuItem::About` menu on Linux.
  **Breaking change:** The About variant now uses an struct instead of a string.
  - [84c677fd](https://github.com/tauri-apps/tao/commit/84c677fd13234c81bbbe63b25d7dc563825c7829) refactor: fix and enhance the about menu on Linux ([#347](https://github.com/tauri-apps/tao/pull/347)) on 2022-03-25
- Fixes the About menu on Linux not being shown.
  - [84c677fd](https://github.com/tauri-apps/tao/commit/84c677fd13234c81bbbe63b25d7dc563825c7829) refactor: fix and enhance the about menu on Linux ([#347](https://github.com/tauri-apps/tao/pull/347)) on 2022-03-25
- Properly fire `WindowEvent::Destroyed` on Linux when the `Window` is dropped.
  - [cdd4ac32](https://github.com/tauri-apps/tao/commit/cdd4ac3281ad9c2cf15561e8cf3110ed34ae93f0) fix(events): properly fire `WindowEvent::Destroyed` on Linux ([#349](https://github.com/tauri-apps/tao/pull/349)) on 2022-03-25
- Properly change the window to fullscreen state if the builder instructs it to use `Fullscreen::Borderless(None)`.
  - [5ecbac19](https://github.com/tauri-apps/tao/commit/5ecbac1958518eaacd263eaaee440f89b2edf122) fix(window): fullscreen on Linux when builder is set to Borderless(None) ([#348](https://github.com/tauri-apps/tao/pull/348)) on 2022-03-25
- Fixes system tray item titles on Windows by forcing the string to be null-terminated.
  - [7f900a16](https://github.com/tauri-apps/tao/commit/7f900a167e0077c354fb26ab6d34ae06591a67c5) fix(tray): force item title string to be null-terminated ([#340](https://github.com/tauri-apps/tao/pull/340)) on 2022-03-09
- Properly fire `WindowEvent::Destroyed` on macOS when the `Window` is dropped.
  - [efd3eecc](https://github.com/tauri-apps/tao/commit/efd3eecc76c45619e36aa8b253316192eefec0d1) fix(window): properly fire `WindowEvent::Destroyed` on macOS ([#350](https://github.com/tauri-apps/tao/pull/350)) on 2022-03-25
- Fix inconsist behaviour when setting menu on mac.
  - [5abdbd1f](https://github.com/tauri-apps/tao/commit/5abdbd1ff7e46f94a686052a65d528725c5647be) Fix inconsist behaviour when setting menu on mac ([#345](https://github.com/tauri-apps/tao/pull/345)) on 2022-03-17

## \[0.6.4]

- Fix a deadlock on Windows when using `Window::set_visible(true)` in the `EventLoop::run` closure.
  - [475e64d2](https://github.com/tauri-apps/tao/commit/475e64d2873c233e60cb74e52e91282d18e13780) fix(Windows): fix a deadlock in `WindowState` ([#338](https://github.com/tauri-apps/tao/pull/338)) on 2022-03-06
- On Windows, apply maximize state before minimize. Fixes `Window::set_minimized` not working when the window is maximized.
  - [11dac102](https://github.com/tauri-apps/tao/commit/11dac10241330c30aae660a2621d43ee5eb3775d) fix(windows): apply maximize state before minimize ([#334](https://github.com/tauri-apps/tao/pull/334)) on 2022-03-01

## \[0.6.3]

- Revert Global Shortcut fix on Linux. See [#331](https://github.com/tauri-apps/tao/issues/331) for more information.
  - [f5e19e0f](https://github.com/tauri-apps/tao/commit/f5e19e0ff83a65410e18ee7e47cac984f87236c9) Revert "Implement global shortcut on Linux, close #307 (#308)" ([#330](https://github.com/tauri-apps/tao/pull/330)) on 2022-03-01

## \[0.6.2]

- Fixes the `set_fullscreen` implementation on Linux when the `Fullscreen::Borderless` value is set to `None`.
  - [456147de](https://github.com/tauri-apps/tao/commit/456147de99e1135b145447a9c8ebb397d3ecd1e1) fix(linux): fullscreen on current monitor ([#320](https://github.com/tauri-apps/tao/pull/320)) on 2022-02-13

## \[0.6.1]

- Fix global shortcut support on Linux (both x11 and wayland).
  - [9c2841f7](https://github.com/tauri-apps/tao/commit/9c2841f7f5382e92efaa8b7bb137d8d30f3e0338) Implement global shortcut on Linux, close [#307](https://github.com/tauri-apps/tao/pull/307) ([#308](https://github.com/tauri-apps/tao/pull/308)) on 2022-02-10

## \[0.6.0]

- Update to gtk 0.15
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Emit errors when parsing an invalid accelerator from a string.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Add support for more accelerator keys: `,` `-` `.` `=` `;` `/` `\` `'` `` ` `` `[` `]` `Space` `Tab` and `F13`-`F24`
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Increased Borderless window resizing inset.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Update to 2021 edition and msrv to 1.56
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- **Breaking:** Rename the `Exit` variant of `ControlFlow` to `ExitWithCode`, which holds a value to control the exit code after running. Add an `Exit` constant which aliases to `ExitWithCode(0)` instead to avoid major breakage. This shouldn't affect most existing programs.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Fixes the `MenuItem::Quit` behavior on Windows.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Add support for `SPACE` shortcut key on Windows.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- - Fix redrawn event that causing infinite lopp on Linux
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Fix linux native menu items not working.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- - Fix resizing undecorated window on Linux.
- Undecorated window can be resized using touch on Linux.
- [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Fix focus events not firing on Linux
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Add monitor selection when fullscreen on Linux and close possible way to create VideoMode on Linux since gtk doesn't acutally have such feature.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- - Add `RedrawEventsCleared` and `RedrawRequested` on Linux
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Add run_return trait on Linux
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- `window.set_skip_taskbar()` on Linux will now also skip the pager (Alt+Tab), this matches the behavior on Windows.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Update tray dependency version.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Fix deadlock when unregistering shortcut on Linux.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Fire `WindowEvent::Resized` and `WindowEvent::Moved` when window is min/maximized on Linux to align with Windows behavior.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Fix menubar missing on borderless window.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Fix core-video-sys dependency.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Fix linking to the `ColorSync` framework on macOS 10.7, and in newer Rust versions.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Allow more strings to parse to keycode, for example `,` is now parsed as a comma.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- - Update `raw-window-handle` to `0.4`
- Add `raw_window_handle()` implementation on linux.
- [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Fix click events missing whe tray has menu.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Add macOS `show_application()` method
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Add new_any_thread to Unix event loop.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Replace all of the `winapi` crate references with the `windows` crate. The generated bindings are in the `webview2-com-sys` crate to share types with WRY later.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Implement `Clone` for `EventLoopWindowTarget`.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Update the `windows` crate to 0.25.0, which comes with pre-built libraries. Tao no longer depends on `webview2-com-sys` to generate bindings shared with WRY.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Update the `windows` crate to 0.29.0.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Update the `windows` crate to 0.30.0. This version re-introduced a lot of new-types for things like HWND, LRESULT, WPARAM, LPARAM, etc.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Fix using `WindowBuilder::with_visible` and `WindowBuilder::with_maximized` not behaving correctly.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- On Windows, send correct position on system tray events.
  - [0dd71973](https://github.com/tauri-apps/tao/commit/0dd7197326fc885150f07ebc3127fd8452a36f00) Merge next back to dev branch ([#305](https://github.com/tauri-apps/tao/pull/305)) on 2022-02-05
- Add support for more accelerator keys: `,` `-` `.` `=` `;` `/` `\` `'` `` ` `` `[` `]` `Space` `Tab` and `F13`-`F24`
  - [b047ae41](https://github.com/tauri-apps/tao/commit/b047ae41a83b94cb140cb3a3decd1ddb4ea6d405) feat: support accelerator key strings `,` `-` `.` `Space` `Tab` and `F13`-`F24` ([#228](https://github.com/tauri-apps/tao/pull/228)) on 2021-11-15
- Allow more strings to parse to keycode, for example `,` is now parsed as a comma.
  - [f0a3dcee](https://github.com/tauri-apps/tao/commit/f0a3dceec585cbfa6749746fdf59054d17ab4d0b) feat: Allow more strings to parse to keycode ([#229](https://github.com/tauri-apps/tao/pull/229)) on 2021-11-03
- Add macOS `show_application()` method
  - [7e10b0df](https://github.com/tauri-apps/tao/commit/7e10b0dfd5018aa3dc326ca99f663efe92a255df) feat(macos): Add `unhide_application` method, closes [#182](https://github.com/tauri-apps/tao/pull/182) ([#231](https://github.com/tauri-apps/tao/pull/231)) on 2021-11-03

## \[0.5.2]

- Fix missing `Sync` trait on EventLoopProxy. This commit also introduces `crossbeam-channel` crate which could also improve the performance.
  - [6122dc5e](https://github.com/tauri-apps/tao/commit/6122dc5ec2041225a697d9673f350bcc2dda44a3) feat: add crossbeam-channel ([#181](https://github.com/tauri-apps/tao/pull/181)) on 2021-08-12

## \[0.5.1]

- Remove feature flag that break doc builds
  - [324eca05](https://github.com/tauri-apps/tao/commit/324eca05d3075e5610f83d1161e23ace656f586e) Update README.md on 2021-05-08
  - [2a90e63c](https://github.com/tauri-apps/tao/commit/2a90e63c8efde9f291172fd89bc1564bfe336aeb) publish new versions on 2021-05-08
  - [eb6f4f9e](https://github.com/tauri-apps/tao/commit/eb6f4f9e18ae7b7537613653ade4f32cb32c5700) Remove feature flag that breaks doc build ([#178](https://github.com/tauri-apps/tao/pull/178)) on 2021-08-09

## \[0.5.0]

- Move `global_shortcut` mod to the lib root.
  - [6e72e54c](https://github.com/tauri-apps/tao/commit/6e72e54c9bb897a8f4319f3f5ec8f7d96bec75d2) refactor: move `global_shortcut` mod to lib roo ([#145](https://github.com/tauri-apps/tao/pull/145)) on 2021-07-20

- Bump gtk-rs to version 0.14. This also introduces a new feature `ayatana` for developers to use updated
  `libayatana-appindicator` since the original `libappindicator` is no longer maintained.
  - [1c0f5274](https://github.com/tauri-apps/tao/commit/1c0f5274cb7246a80b382ecf7ae3f7e6c858f2f2) chore: bump gtk to v0.14 ([#173](https://github.com/tauri-apps/tao/pull/173)) on 2021-08-06

- Remove Clipboard MenuItem on Linux since they only work on a few sepcific widget.
  - [969052ab](https://github.com/tauri-apps/tao/commit/969052abab84ff6ad7b43f39e444e944b2de862d) fix(linux): remove clipboard menuitems on Linux ([#150](https://github.com/tauri-apps/tao/pull/150)) on 2021-07-21

- Fixes incorrect monitor size on Linux.
  - [eb051931](https://github.com/tauri-apps/tao/commit/eb0519316626d02a00569176f835ec8bc4f7746e) fix(linux): incorrect monitor size, fixes: [#175](https://github.com/tauri-apps/tao/pull/175) ([#176](https://github.com/tauri-apps/tao/pull/176)) on 2021-08-08

- Fix `no key equivalent for Accelerator` for `Space`, `Escape`, `Minus` and `Equal` keycode.
  - [ecd3c405](https://github.com/tauri-apps/tao/commit/ecd3c405cd57852e3a55a9a6539784b99f32bf6c) fix(accelerator): add missing KeyCode to prevent `no key equivalent for Accelerator` ([#148](https://github.com/tauri-apps/tao/pull/148)) on 2021-07-20

- Fix incorrect macOS Redo and Close Window shortcuts
  - [f4d718a8](https://github.com/tauri-apps/tao/commit/f4d718a89a03ca0ae53817e7895acb953175ef6b) fix(macos): Fix incorrect Redo and CloseWindow accelerators ([#166](https://github.com/tauri-apps/tao/pull/166)) on 2021-08-03

- - Support [macOS tray icon template](https://developer.apple.com/documentation/appkit/nsimage/1520017-template?language=objc) to adjust automatically based on taskbar color.

- Images you mark as template images should consist of only black and clear colors. You can use the alpha channel in the image to adjust the opacity of black content, however.

- [577458c4](https://github.com/tauri-apps/tao/commit/577458c4588d667524ef483956d85a4420402bb2) feat(tray): Support macOS icon template ([#162](https://github.com/tauri-apps/tao/pull/162)) on 2021-07-29

- macOS: Add `with_parent_window()` on `WindowBuilder`.
  - [73c7aac7](https://github.com/tauri-apps/tao/commit/73c7aac7b04355862c68a42179f52c9f9dec727d) feat(macOS): Allow creation of child Window ([#160](https://github.com/tauri-apps/tao/pull/160)) on 2021-08-04

- Removed `SystemTrayExtWindows::remove()`, the icon will be automatically removed when `SystemTray` is dropped.
  - [cc9d2b17](https://github.com/tauri-apps/tao/commit/cc9d2b1726d378ee28ca69e6e5e2255c2d71f214) refactor: refactor `system_tray` impl on windows ([#153](https://github.com/tauri-apps/tao/pull/153)) on 2021-07-22

- Add `MenuItem::SelectAll` implementation on windows.
  - [222adeb2](https://github.com/tauri-apps/tao/commit/222adeb238c290b8101507aed976869c34a8d4ab) feat(window): add `Select all` native menu item ([#146](https://github.com/tauri-apps/tao/pull/146)) on 2021-07-21

- Add flags to support all other possible unix systems.
  - [546f51a3](https://github.com/tauri-apps/tao/commit/546f51a3974c15af0d6c84525d3c8e448911f8a8) Add flags to support other unix systems. ([#142](https://github.com/tauri-apps/tao/pull/142)) on 2021-07-20

- Fix confliction between `set_skip_taksbar(true)` and `set_visible(false)`.
  - [226e6611](https://github.com/tauri-apps/tao/commit/226e66113aa96495d50e8c6024afbf991f41dc7d) fix(Windows): conflict between taskbar and visible ([#172](https://github.com/tauri-apps/tao/pull/172)) on 2021-08-06

## \[0.4.0]

- On Windows, Allow resizing of `decorations: false` aka borderless window.
  - [f35dd03d](https://github.com/tauri-apps/tao/commit/f35dd03dc6f15d51fb348c6b404c195ba2401339) fix(windows): fix aero-snap and resizing of borderless window, fixes [#103](https://github.com/tauri-apps/tao/pull/103) [#104](https://github.com/tauri-apps/tao/pull/104) ([#110](https://github.com/tauri-apps/tao/pull/110)) on 2021-07-07
- Do not close the window on `CloseRequested` event and let the user handle it, keeping compatibility with macOS and Windows behavior.
  - [ea7330ef](https://github.com/tauri-apps/tao/commit/ea7330eff77dba8ab0b5e65d82313a7b15733190) fix(linux): do not close window on `CloseRequested` event ([#114](https://github.com/tauri-apps/tao/pull/114)) on 2021-07-05
- On Windows, fix Aero-Snap for `decorations: false` aka borderless window.
  - [f35dd03d](https://github.com/tauri-apps/tao/commit/f35dd03dc6f15d51fb348c6b404c195ba2401339) fix(windows): fix aero-snap and resizing of borderless window, fixes [#103](https://github.com/tauri-apps/tao/pull/103) [#104](https://github.com/tauri-apps/tao/pull/104) ([#110](https://github.com/tauri-apps/tao/pull/110)) on 2021-07-07
- Implement `MonitorHandle` and related methods on Linux.
  - [6fcfa629](https://github.com/tauri-apps/tao/commit/6fcfa62959800e6205068e74fa8648b5e12c6103) feat(linux): implement `MonitorHandle` and related methods ([#125](https://github.com/tauri-apps/tao/pull/125)) on 2021-07-12
- Add `is_menu_visilbe` getter on `Window`
  - [308411ca](https://github.com/tauri-apps/tao/commit/308411caeacc3b7c701d8d857964248a3411dfaa) feat: add `is_menu_visible` ([#108](https://github.com/tauri-apps/tao/pull/108)) on 2021-07-06
- On macOS, make sure the `set_focus` is triggered even if the window is not visible.
  - [3da167aa](https://github.com/tauri-apps/tao/commit/3da167aad9dad8ec2e3b3af52175a74a5ef07b99) fix(macos): `set_focus` should be triggered even if the window isn't visible ([#128](https://github.com/tauri-apps/tao/pull/128)) on 2021-07-14
- Fix `with_visible(bool)` in `WindowBuilder` for macOS.
  - [a0ac7075](https://github.com/tauri-apps/tao/commit/a0ac7075bdc5b9e37900c0f38b97a86071ce1dfd) fix(macos): Window state (`visible`) ([#119](https://github.com/tauri-apps/tao/pull/119)) on 2021-07-06
- Mark enums as `#[non_exhaustive]` to prevent breaking changes on enum update.
  - [9b906f50](https://github.com/tauri-apps/tao/commit/9b906f508477f0a67cc3b853909c24fe754b86c9) refactor: add `#[non_exhaustive]` attributes to enums ([#90](https://github.com/tauri-apps/tao/pull/90)) on 2021-07-07
- Remove `with_focus` and `focus` field in `WindowAttribute`. Use `set_focus` instead in most cases.
  - [e2399bc9](https://github.com/tauri-apps/tao/commit/e2399bc92642601d999a2579c0626dfe017a262c) Remove `with_focus` and `focus` field in `WindowAttribute` ([#121](https://github.com/tauri-apps/tao/pull/121)) on 2021-07-06
- Revert d344825 and move `set_skip_taskbar` back behind a `WindowExtWindows` and `WindowExtUnix`.
  - [a641d3a3](https://github.com/tauri-apps/tao/commit/a641d3a317c132661abf08723e4f87fb515e00ed) refactor: Revert d344825, move `set_skip_taskbar` behind platform-ext ([#118](https://github.com/tauri-apps/tao/pull/118)) on 2021-07-06
- `SystemTray` expose `set_menu` to update the system tray menu once created.
  - [578dd23e](https://github.com/tauri-apps/tao/commit/578dd23e02bbc63a2d2362b823730c470c1c029c) feat: implement `set_menu` for system tray ([#106](https://github.com/tauri-apps/tao/pull/106)) on 2021-07-14
- Only show window behaviour when it is visible. winuser::ShowWindow will show the window and make with_visible(false) obsolete.
  - [ff0903f6](https://github.com/tauri-apps/tao/commit/ff0903f62b7e206fd9018a7140f5d2729d4ab8ba) Only show window behaviour when it is visible ([#126](https://github.com/tauri-apps/tao/pull/126)) on 2021-07-14
- Add `with_skip_taskbar` behind `WindowBuilderExtWindows` and `WindowBuilderExtUnix`.
  - [e7cdb950](https://github.com/tauri-apps/tao/commit/e7cdb950c719a0efd93538324ba8773dd2c7abcc) feat(taskbar): add `with_skip_taskbar` for windows and linux ([#127](https://github.com/tauri-apps/tao/pull/127)) on 2021-07-14

## \[0.3.1]

- Add `window_id` to `MenuEvent`.
  - [96651dcc](https://github.com/tauri-apps/tao/commit/96651dccd2f81229a6a7d8a0fc8ffb122c099b30) feat(menu): Add `window_id` to `MenuEvent` ([#89](https://github.com/tauri-apps/tao/pull/89)) on 2021-06-22
- Prevent duplicate `MenuEvent` on window menu in Windows.
  - [8cf4033f](https://github.com/tauri-apps/tao/commit/8cf4033f81e1b0cdfd88e86cf34cdcd174e1a3a9) fix(windows): menu event ([#91](https://github.com/tauri-apps/tao/pull/91)) on 2021-06-22

## \[0.3.0]

- Drop the event callback before exiting on macOS.
  - [52ebebbc](https://github.com/tauri-apps/tao/commit/52ebebbc5cc2913bb4b50ea7743e9f6be21e6657) Drop the event callback before exiting ([#86](https://github.com/tauri-apps/tao/pull/86)) on 2021-06-18
- Add `clipboard` api exposing `read_text` and `write_text`.
  - [cf22c902](https://github.com/tauri-apps/tao/commit/cf22c902d4c961be0f6cfba6a8c865e11073b027) feat: clipboard api ([#85](https://github.com/tauri-apps/tao/pull/85)) on 2021-06-21
- Fix LoopDestroyed to really exit the application.
  - [55e52a91](https://github.com/tauri-apps/tao/commit/55e52a9149adf37131e365fa667a97f9a24f1e4f) Fix LoopDestroy condition to really exit the app on 2021-06-01
- Implement all control flow variants
  - [16e2ac06](https://github.com/tauri-apps/tao/commit/16e2ac06c2f0c180a17fb8021005dec51a57af49) Add change file on 2021-05-19
- Add checks before focusing window
  - [1bd3b1c0](https://github.com/tauri-apps/tao/commit/1bd3b1c0fbdbf4e8a9bfade34a7e217f26114859) Add change file on 2021-05-22
- Add `is_visible` getter on `Window`
  - [c402a38b](https://github.com/tauri-apps/tao/commit/c402a38b1bbf240f4d2553c3f2b4edf86f03c270) feat: Add `is_visible` getter to `Window` ([#61](https://github.com/tauri-apps/tao/pull/61)) on 2021-05-27
- **Breaking change**: New keyboard API, including `Accelerator` and `GlobalShortcut`.

`WindowEvent::ModifiersChanged` is emitted when a new keyboard modifier is pressed. This is your responsibility to keep a local state. When the modifier is released, `ModifiersState::empty()` is emitted.

`WindowEvent::KeyboardInput` as been refactored and is exposing the event `KeyEvent`.

All menus (`ContextMenu` and `MenuBar`), now includes `Accelerator` support on Windows, macOS and Linux.

New modules available: `keyboard`, `accelerator` and `platform::global_shortcut`.

*Please refer to the docs and examples for more details.*

- [01fc43b0](https://github.com/tauri-apps/tao/commit/01fc43b05ea41463d512c0e3497971edc543ac9d) refactor: keyboards events ([#82](https://github.com/tauri-apps/tao/pull/82)) on 2021-06-21
- **Breaking change**: New menu/tray API.

System tray now expose `set_icon()` to update the tray icon after initialization. The `system_tray::SystemTrayBuilder` has been moved to the root of the package as a module and available on Windows, Linux and macOS, only when the `tray` feature is enabled. Windows expose a `remove()` function available with `SystemTrayExtWindows`.

Menu builder has been rebuilt from scratch, exposing 2 different types, `ContextMenu` and `MenuBar`.

Please refer to the docs and examples for more details.

- [7546dbd1](https://github.com/tauri-apps/tao/commit/7546dbd157b5387249337c5166849e2868c0ab7c) refactor: menu & tray ([#77](https://github.com/tauri-apps/tao/pull/77)) on 2021-06-03
- Fix match branch of run loop observer on iOS.
  - [4e9fede6](https://github.com/tauri-apps/tao/commit/4e9fede6b63d4cf60ea0a6b974a61a00bd2b47df) Add change file on 2021-05-23
- - `skip_taskbar` is renamed to `set_skip_taskbar`.
- `set_skip_taskbar` is now available on `Window` and is no longer behind a PlatformExt.
- `set_skip_taskbar` takes a boolean to either show or hide the window icon from the taskbar.
- Add `with_skip_taskbar` to `WindowBuilder`.
- [c0aac091](https://github.com/tauri-apps/tao/commit/c0aac0911e77b8b0b709d20fa9d1a3297b38e7ee) add `with_skip_taskbar` on 2021-05-29
- Add `skip_taskbar` implementation for windows
  - [83341701](https://github.com/tauri-apps/tao/commit/83341701843122f57139cf7583db345385b81d3c) feat: add `skip_taskabr` impl for windows ([#78](https://github.com/tauri-apps/tao/pull/78)) on 2021-05-29

## \[0.2.6]

- Add `is_decorated` getter on `Window`
  - [8237e2f3](https://github.com/tauri-apps/tao/commit/8237e2f3d54a012f3a89aea84112fb125863b582) add changefile on 2021-05-13
- Add `is_resizable` getter on `Window`
  - [c87f3bf9](https://github.com/tauri-apps/tao/commit/c87f3bf925df3692a44982c72cfea7484758bccc) add changefile on 2021-05-13
- Fix panic from borrowing in event loop on linux.
  - [12d7ccbc](https://github.com/tauri-apps/tao/commit/12d7ccbc4dd31ae35a45dd56542c5377d5235ecc) Fix event loop on linux on 2021-05-17
- Implement `set_focus()` and `with_focus()` for macOS, Windows and Linux.
  - [448e4c17](https://github.com/tauri-apps/tao/commit/448e4c17e2ba58257b6c23041faeae0551189536) Add change file on 2021-05-07

## \[0.2.5]

- Fix Priority import on Linux.
  - [20128896](https://github.com/tauri-apps/tao/commit/201288960e6af87a2e4ae9a75b3d53a874edbd3e) Add change file on 2021-05-17

## \[0.2.4]

- Refactor control flow implementation to wait.
  - [f5514f04](https://github.com/tauri-apps/tao/commit/f5514f04819f13493e75dfc844c7b06ec17bb071) Add change file on 2021-05-15

## \[0.2.3]

- Split feature flags (menu and tray).
  - [0035ac31](https://github.com/tauri-apps/tao/commit/0035ac31ea32fd1b04339a678e321411b779a5b6) Add changefile on 2021-05-10

## \[0.2.2]

- Add dox flag to skip link lib when building doc.
  - [565114c1](https://github.com/tauri-apps/tao/commit/565114c16a27b321e326195ad1f248ffa721c3a3) Add dox flag on 2021-05-09

## \[0.2.1]

- Update covector script to fix doc build.
  - [25f291f2](https://github.com/tauri-apps/tao/commit/25f291f2385b9505b31b8db2c74fd2aad4b7d699) Update covector script to fix doc build on 2021-05-09

## \[0.2.0]

- Update README and bump version.
  - [324eca05](https://github.com/tauri-apps/tao/commit/324eca05d3075e5610f83d1161e23ace656f586e) Update README.md on 2021-05-08
- Implement menu item varients for Linux.
  - [0637570f](https://github.com/tauri-apps/tao/commit/0637570f151f3ad6d5fcaa37dbacb84a5b53624a) Add change file on 2021-05-06
- Implement status bar on Linux.
  - [e17bce40](https://github.com/tauri-apps/tao/commit/e17bce40df35f4b8e9ff018a5e9b14f446eee899) Add change file on 2021-05-07
  - [86743720](https://github.com/tauri-apps/tao/commit/867437209f820f461239505201e7b21d8d66495c) \[skip ci] Update change file description on 2021-05-07
- Implement basic menu builder for macOS, Windows and Linux.
  - [ecd528d6](https://github.com/tauri-apps/tao/commit/ecd528d6c137c7d3ca8d5f16bb3f082b961db6d9) Add change file on 2021-05-04
  - [47640216](https://github.com/tauri-apps/tao/commit/476402167ce7209b57a180453733132b777c44f5) \[skip ci] Update changelog on 2021-05-05
- Add menu feature flag and rename status bar to system tray.
  - [06d95ad0](https://github.com/tauri-apps/tao/commit/06d95ad03c15b3da1601007ef1b81ea90310d177) Cargo fmt & clippy on 2021-05-08
- Implement basic menu builder for macOS, Windows and Linux.
  - [63868365](https://github.com/tauri-apps/tao/commit/63868365613bfd3f6c6d2155e9b3120f7fb9962e) Add change file on 2021-05-05
  - [94254074](https://github.com/tauri-apps/tao/commit/942540745e3c8365ac4d0813e9ae40dc7090a2cd) \[ci skip] Update changelog on 2021-05-06
  - [e17bce40](https://github.com/tauri-apps/tao/commit/e17bce40df35f4b8e9ff018a5e9b14f446eee899) Add change file on 2021-05-07
  - [86743720](https://github.com/tauri-apps/tao/commit/867437209f820f461239505201e7b21d8d66495c) \[skip ci] Update change file description on 2021-05-07
