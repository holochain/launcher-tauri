var ee = Object.defineProperty, te = Object.defineProperties;
var ae = Object.getOwnPropertyDescriptors;
var L = Object.getOwnPropertySymbols;
var re = Object.prototype.hasOwnProperty, ne = Object.prototype.propertyIsEnumerable;
var T = (t, e, a) => e in t ? ee(t, e, { enumerable: !0, configurable: !0, writable: !0, value: a }) : t[e] = a, u = (t, e) => {
  for (var a in e || (e = {}))
    re.call(e, a) && T(t, a, e[a]);
  if (L)
    for (var a of L(e))
      ne.call(e, a) && T(t, a, e[a]);
  return t;
}, d = (t, e) => te(t, ae(e));
var ie = Object.defineProperty, l = (t, e) => {
  for (var a in e)
    ie(t, a, { get: e[a], enumerable: !0 });
}, se = {};
l(se, { convertFileSrc: () => le, invoke: () => v, transformCallback: () => c });
function oe() {
  return window.crypto.getRandomValues(new Uint32Array(1))[0];
}
function c(t, e = !1) {
  let a = oe(), n = `_${a}`;
  return Object.defineProperty(window, n, { value: (i) => (e && Reflect.deleteProperty(window, n), t == null ? void 0 : t(i)), writable: !1, configurable: !0 }), a;
}
async function v(t, e = {}) {
  return new Promise((a, n) => {
    let i = c((o) => {
      a(o), Reflect.deleteProperty(window, `_${s}`);
    }, !0), s = c((o) => {
      n(o), Reflect.deleteProperty(window, `_${i}`);
    }, !0);
    window.__TAURI_IPC__(u({ cmd: t, callback: i, error: s }, e));
  });
}
function le(t, e = "asset") {
  let a = encodeURIComponent(t);
  return navigator.userAgent.includes("Windows") ? `https://${e}.localhost/${a}` : `${e}://localhost/${a}`;
}
async function r(t) {
  return v("tauri", t);
}
var ue = {};
l(ue, { TauriEvent: () => x, emit: () => D, listen: () => F, once: () => R });
async function S(t, e) {
  return r({ __tauriModule: "Event", message: { cmd: "unlisten", event: t, eventId: e } });
}
async function z(t, e, a) {
  await r({ __tauriModule: "Event", message: { cmd: "emit", event: t, windowLabel: e, payload: a } });
}
async function P(t, e, a) {
  return r({ __tauriModule: "Event", message: { cmd: "listen", event: t, windowLabel: e, handler: c(a) } }).then((n) => async () => S(t, n));
}
async function O(t, e, a) {
  return P(t, e, (n) => {
    a(n), S(t, n.id).catch(() => {
    });
  });
}
var x = ((t) => (t.WINDOW_RESIZED = "tauri://resize", t.WINDOW_MOVED = "tauri://move", t.WINDOW_CLOSE_REQUESTED = "tauri://close-requested", t.WINDOW_CREATED = "tauri://window-created", t.WINDOW_DESTROYED = "tauri://destroyed", t.WINDOW_FOCUS = "tauri://focus", t.WINDOW_BLUR = "tauri://blur", t.WINDOW_SCALE_FACTOR_CHANGED = "tauri://scale-change", t.WINDOW_THEME_CHANGED = "tauri://theme-changed", t.WINDOW_FILE_DROP = "tauri://file-drop", t.WINDOW_FILE_DROP_HOVER = "tauri://file-drop-hover", t.WINDOW_FILE_DROP_CANCELLED = "tauri://file-drop-cancelled", t.MENU = "tauri://menu", t.CHECK_UPDATE = "tauri://update", t.UPDATE_AVAILABLE = "tauri://update-available", t.INSTALL_UPDATE = "tauri://update-install", t.STATUS_UPDATE = "tauri://update-status", t.DOWNLOAD_PROGRESS = "tauri://update-download-progress", t))(x || {});
async function F(t, e) {
  return P(t, null, e);
}
async function R(t, e) {
  return O(t, null, e);
}
async function D(t, e) {
  return z(t, void 0, e);
}
var ce = {};
l(ce, { checkUpdate: () => me, installUpdate: () => de, onUpdaterEvent: () => $ });
async function $(t) {
  return F("tauri://update-status", (e) => {
    t(e == null ? void 0 : e.payload);
  });
}
async function de() {
  let t;
  function e() {
    t && t(), t = void 0;
  }
  return new Promise((a, n) => {
    function i(s) {
      if (s.error) {
        e(), n(s.error);
        return;
      }
      s.status === "DONE" && (e(), a());
    }
    $(i).then((s) => {
      t = s;
    }).catch((s) => {
      throw e(), s;
    }), D("tauri://update-install").catch((s) => {
      throw e(), s;
    });
  });
}
async function me() {
  let t;
  function e() {
    t && t(), t = void 0;
  }
  return new Promise((a, n) => {
    function i(o) {
      e(), a({ manifest: o, shouldUpdate: !0 });
    }
    function s(o) {
      if (o.error) {
        e(), n(o.error);
        return;
      }
      o.status === "UPTODATE" && (e(), a({ shouldUpdate: !1 }));
    }
    R("tauri://update-available", (o) => {
      i(o == null ? void 0 : o.payload);
    }).catch((o) => {
      throw e(), o;
    }), $(s).then((o) => {
      t = o;
    }).catch((o) => {
      throw e(), o;
    }), D("tauri://update").catch((o) => {
      throw e(), o;
    });
  });
}
var he = {};
l(he, { CloseRequestedEvent: () => q, LogicalPosition: () => I, LogicalSize: () => N, PhysicalPosition: () => g, PhysicalSize: () => _, UserAttentionType: () => k, WebviewWindow: () => m, WebviewWindowHandle: () => U, WindowManager: () => j, appWindow: () => M, availableMonitors: () => ge, currentMonitor: () => pe, getAll: () => b, getCurrent: () => ye, primaryMonitor: () => _e });
var N = class {
  constructor(e, a) {
    this.type = "Logical", this.width = e, this.height = a;
  }
}, _ = class {
  constructor(e, a) {
    this.type = "Physical", this.width = e, this.height = a;
  }
  toLogical(e) {
    return new N(this.width / e, this.height / e);
  }
}, I = class {
  constructor(e, a) {
    this.type = "Logical", this.x = e, this.y = a;
  }
}, g = class {
  constructor(e, a) {
    this.type = "Physical", this.x = e, this.y = a;
  }
  toLogical(e) {
    return new I(this.x / e, this.y / e);
  }
}, k = ((t) => (t[t.Critical = 1] = "Critical", t[t.Informational = 2] = "Informational", t))(k || {});
function ye() {
  return new m(window.__TAURI_METADATA__.__currentWindow.label, { skip: !0 });
}
function b() {
  return window.__TAURI_METADATA__.__windows.map((t) => new m(t.label, { skip: !0 }));
}
var E = ["tauri://created", "tauri://error"], U = class {
  constructor(e) {
    this.label = e, this.listeners = /* @__PURE__ */ Object.create(null);
  }
  async listen(e, a) {
    return this._handleTauriEvent(e, a) ? Promise.resolve(() => {
      let n = this.listeners[e];
      n.splice(n.indexOf(a), 1);
    }) : P(e, this.label, a);
  }
  async once(e, a) {
    return this._handleTauriEvent(e, a) ? Promise.resolve(() => {
      let n = this.listeners[e];
      n.splice(n.indexOf(a), 1);
    }) : O(e, this.label, a);
  }
  async emit(e, a) {
    if (E.includes(e)) {
      for (let n of this.listeners[e] || [])
        n({ event: e, id: -1, windowLabel: this.label, payload: a });
      return Promise.resolve();
    }
    return z(e, this.label, a);
  }
  _handleTauriEvent(e, a) {
    return E.includes(e) ? (e in this.listeners ? this.listeners[e].push(a) : this.listeners[e] = [a], !0) : !1;
  }
}, j = class extends U {
  async scaleFactor() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "scaleFactor" } } } });
  }
  async innerPosition() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "innerPosition" } } } }).then(({ x: e, y: a }) => new g(e, a));
  }
  async outerPosition() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "outerPosition" } } } }).then(({ x: e, y: a }) => new g(e, a));
  }
  async innerSize() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "innerSize" } } } }).then(({ width: e, height: a }) => new _(e, a));
  }
  async outerSize() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "outerSize" } } } }).then(({ width: e, height: a }) => new _(e, a));
  }
  async isFullscreen() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "isFullscreen" } } } });
  }
  async isMinimized() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "isMinimized" } } } });
  }
  async isMaximized() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "isMaximized" } } } });
  }
  async isFocused() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "isFocused" } } } });
  }
  async isDecorated() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "isDecorated" } } } });
  }
  async isResizable() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "isResizable" } } } });
  }
  async isMaximizable() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "isMaximizable" } } } });
  }
  async isMinimizable() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "isMinimizable" } } } });
  }
  async isClosable() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "isClosable" } } } });
  }
  async isVisible() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "isVisible" } } } });
  }
  async title() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "title" } } } });
  }
  async theme() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "theme" } } } });
  }
  async center() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "center" } } } });
  }
  async requestUserAttention(e) {
    let a = null;
    return e && (e === 1 ? a = { type: "Critical" } : a = { type: "Informational" }), r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "requestUserAttention", payload: a } } } });
  }
  async setResizable(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setResizable", payload: e } } } });
  }
  async setMaximizable(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setMaximizable", payload: e } } } });
  }
  async setMinimizable(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setMinimizable", payload: e } } } });
  }
  async setClosable(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setClosable", payload: e } } } });
  }
  async setTitle(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setTitle", payload: e } } } });
  }
  async maximize() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "maximize" } } } });
  }
  async unmaximize() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "unmaximize" } } } });
  }
  async toggleMaximize() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "toggleMaximize" } } } });
  }
  async minimize() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "minimize" } } } });
  }
  async unminimize() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "unminimize" } } } });
  }
  async show() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "show" } } } });
  }
  async hide() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "hide" } } } });
  }
  async close() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "close" } } } });
  }
  async setDecorations(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setDecorations", payload: e } } } });
  }
  async setAlwaysOnTop(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setAlwaysOnTop", payload: e } } } });
  }
  async setContentProtected(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setContentProtected", payload: e } } } });
  }
  async setSize(e) {
    if (!e || e.type !== "Logical" && e.type !== "Physical")
      throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setSize", payload: { type: e.type, data: { width: e.width, height: e.height } } } } } });
  }
  async setMinSize(e) {
    if (e && e.type !== "Logical" && e.type !== "Physical")
      throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setMinSize", payload: e ? { type: e.type, data: { width: e.width, height: e.height } } : null } } } });
  }
  async setMaxSize(e) {
    if (e && e.type !== "Logical" && e.type !== "Physical")
      throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setMaxSize", payload: e ? { type: e.type, data: { width: e.width, height: e.height } } : null } } } });
  }
  async setPosition(e) {
    if (!e || e.type !== "Logical" && e.type !== "Physical")
      throw new Error("the `position` argument must be either a LogicalPosition or a PhysicalPosition instance");
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setPosition", payload: { type: e.type, data: { x: e.x, y: e.y } } } } } });
  }
  async setFullscreen(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setFullscreen", payload: e } } } });
  }
  async setFocus() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setFocus" } } } });
  }
  async setIcon(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setIcon", payload: { icon: typeof e == "string" ? e : Array.from(e) } } } } });
  }
  async setSkipTaskbar(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setSkipTaskbar", payload: e } } } });
  }
  async setCursorGrab(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setCursorGrab", payload: e } } } });
  }
  async setCursorVisible(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setCursorVisible", payload: e } } } });
  }
  async setCursorIcon(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setCursorIcon", payload: e } } } });
  }
  async setCursorPosition(e) {
    if (!e || e.type !== "Logical" && e.type !== "Physical")
      throw new Error("the `position` argument must be either a LogicalPosition or a PhysicalPosition instance");
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setCursorPosition", payload: { type: e.type, data: { x: e.x, y: e.y } } } } } });
  }
  async setIgnoreCursorEvents(e) {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "setIgnoreCursorEvents", payload: e } } } });
  }
  async startDragging() {
    return r({ __tauriModule: "Window", message: { cmd: "manage", data: { label: this.label, cmd: { type: "startDragging" } } } });
  }
  async onResized(e) {
    return this.listen("tauri://resize", (a) => {
      a.payload = V(a.payload), e(a);
    });
  }
  async onMoved(e) {
    return this.listen("tauri://move", (a) => {
      a.payload = H(a.payload), e(a);
    });
  }
  async onCloseRequested(e) {
    return this.listen("tauri://close-requested", (a) => {
      let n = new q(a);
      Promise.resolve(e(n)).then(() => {
        if (!n.isPreventDefault())
          return this.close();
      });
    });
  }
  async onFocusChanged(e) {
    let a = await this.listen("tauri://focus", (i) => {
      e(d(u({}, i), { payload: !0 }));
    }), n = await this.listen("tauri://blur", (i) => {
      e(d(u({}, i), { payload: !1 }));
    });
    return () => {
      a(), n();
    };
  }
  async onScaleChanged(e) {
    return this.listen("tauri://scale-change", e);
  }
  async onMenuClicked(e) {
    return this.listen("tauri://menu", e);
  }
  async onFileDropEvent(e) {
    let a = await this.listen("tauri://file-drop", (s) => {
      e(d(u({}, s), { payload: { type: "drop", paths: s.payload } }));
    }), n = await this.listen("tauri://file-drop-hover", (s) => {
      e(d(u({}, s), { payload: { type: "hover", paths: s.payload } }));
    }), i = await this.listen("tauri://file-drop-cancelled", (s) => {
      e(d(u({}, s), { payload: { type: "cancel" } }));
    });
    return () => {
      a(), n(), i();
    };
  }
  async onThemeChanged(e) {
    return this.listen("tauri://theme-changed", e);
  }
}, q = class {
  constructor(e) {
    this._preventDefault = !1, this.event = e.event, this.windowLabel = e.windowLabel, this.id = e.id;
  }
  preventDefault() {
    this._preventDefault = !0;
  }
  isPreventDefault() {
    return this._preventDefault;
  }
}, m = class extends j {
  constructor(e, a = {}) {
    super(e), a != null && a.skip || r({ __tauriModule: "Window", message: { cmd: "createWebview", data: { options: u({ label: e }, a) } } }).then(async () => this.emit("tauri://created")).catch(async (n) => this.emit("tauri://error", n));
  }
  static getByLabel(e) {
    return b().some((a) => a.label === e) ? new m(e, { skip: !0 }) : null;
  }
  static async getFocusedWindow() {
    for (let e of b())
      if (await e.isFocused())
        return e;
    return null;
  }
}, M;
"__TAURI_METADATA__" in window ? M = new m(window.__TAURI_METADATA__.__currentWindow.label, { skip: !0 }) : (console.warn(`Could not find "window.__TAURI_METADATA__". The "appWindow" value will reference the "main" window label.
Note that this is not an issue if running this frontend on a browser instead of a Tauri window.`), M = new m("main", { skip: !0 }));
function W(t) {
  return t === null ? null : { name: t.name, scaleFactor: t.scaleFactor, position: H(t.position), size: V(t.size) };
}
function H(t) {
  return new g(t.x, t.y);
}
function V(t) {
  return new _(t.width, t.height);
}
async function pe() {
  return r({ __tauriModule: "Window", message: { cmd: "manage", data: { cmd: { type: "currentMonitor" } } } }).then(W);
}
async function _e() {
  return r({ __tauriModule: "Window", message: { cmd: "manage", data: { cmd: { type: "primaryMonitor" } } } }).then(W);
}
async function ge() {
  return r({ __tauriModule: "Window", message: { cmd: "manage", data: { cmd: { type: "availableMonitors" } } } }).then((t) => t.map(W));
}
var fe = {};
l(fe, { isPermissionGranted: () => we, requestPermission: () => be, sendNotification: () => Me });
async function we() {
  return window.Notification.permission !== "default" ? Promise.resolve(window.Notification.permission === "granted") : r({ __tauriModule: "Notification", message: { cmd: "isNotificationPermissionGranted" } });
}
async function be() {
  return window.Notification.requestPermission();
}
function Me(t) {
  typeof t == "string" ? new window.Notification(t) : new window.Notification(t.title, t);
}
function A() {
  return navigator.appVersion.includes("Win");
}
var ve = {};
l(ve, { EOL: () => Pe, arch: () => Ae, locale: () => Te, platform: () => De, tempdir: () => Le, type: () => We, version: () => $e });
var Pe = A() ? `\r
` : `
`;
async function De() {
  return r({ __tauriModule: "Os", message: { cmd: "platform" } });
}
async function $e() {
  return r({ __tauriModule: "Os", message: { cmd: "version" } });
}
async function We() {
  return r({ __tauriModule: "Os", message: { cmd: "osType" } });
}
async function Ae() {
  return r({ __tauriModule: "Os", message: { cmd: "arch" } });
}
async function Le() {
  return r({ __tauriModule: "Os", message: { cmd: "tempdir" } });
}
async function Te() {
  return r({ __tauriModule: "Os", message: { cmd: "locale" } });
}
var Ee = {};
l(Ee, { BaseDirectory: () => f, Dir: () => f, copyFile: () => Re, createDir: () => xe, exists: () => ke, readBinaryFile: () => Se, readDir: () => Oe, readTextFile: () => Ce, removeDir: () => Fe, removeFile: () => Ne, renameFile: () => Ie, writeBinaryFile: () => ze, writeFile: () => C, writeTextFile: () => C });
var f = ((t) => (t[t.Audio = 1] = "Audio", t[t.Cache = 2] = "Cache", t[t.Config = 3] = "Config", t[t.Data = 4] = "Data", t[t.LocalData = 5] = "LocalData", t[t.Desktop = 6] = "Desktop", t[t.Document = 7] = "Document", t[t.Download = 8] = "Download", t[t.Executable = 9] = "Executable", t[t.Font = 10] = "Font", t[t.Home = 11] = "Home", t[t.Picture = 12] = "Picture", t[t.Public = 13] = "Public", t[t.Runtime = 14] = "Runtime", t[t.Template = 15] = "Template", t[t.Video = 16] = "Video", t[t.Resource = 17] = "Resource", t[t.App = 18] = "App", t[t.Log = 19] = "Log", t[t.Temp = 20] = "Temp", t[t.AppConfig = 21] = "AppConfig", t[t.AppData = 22] = "AppData", t[t.AppLocalData = 23] = "AppLocalData", t[t.AppCache = 24] = "AppCache", t[t.AppLog = 25] = "AppLog", t))(f || {});
async function Ce(t, e = {}) {
  return r({ __tauriModule: "Fs", message: { cmd: "readTextFile", path: t, options: e } });
}
async function Se(t, e = {}) {
  let a = await r({ __tauriModule: "Fs", message: { cmd: "readFile", path: t, options: e } });
  return Uint8Array.from(a);
}
async function C(t, e, a) {
  typeof a == "object" && Object.freeze(a), typeof t == "object" && Object.freeze(t);
  let n = { path: "", contents: "" }, i = a;
  return typeof t == "string" ? n.path = t : (n.path = t.path, n.contents = t.contents), typeof e == "string" ? n.contents = e != null ? e : "" : i = e, r({ __tauriModule: "Fs", message: { cmd: "writeFile", path: n.path, contents: Array.from(new TextEncoder().encode(n.contents)), options: i } });
}
async function ze(t, e, a) {
  typeof a == "object" && Object.freeze(a), typeof t == "object" && Object.freeze(t);
  let n = { path: "", contents: [] }, i = a;
  return typeof t == "string" ? n.path = t : (n.path = t.path, n.contents = t.contents), e && "dir" in e ? i = e : typeof t == "string" && (n.contents = e != null ? e : []), r({ __tauriModule: "Fs", message: { cmd: "writeFile", path: n.path, contents: Array.from(n.contents instanceof ArrayBuffer ? new Uint8Array(n.contents) : n.contents), options: i } });
}
async function Oe(t, e = {}) {
  return r({ __tauriModule: "Fs", message: { cmd: "readDir", path: t, options: e } });
}
async function xe(t, e = {}) {
  return r({ __tauriModule: "Fs", message: { cmd: "createDir", path: t, options: e } });
}
async function Fe(t, e = {}) {
  return r({ __tauriModule: "Fs", message: { cmd: "removeDir", path: t, options: e } });
}
async function Re(t, e, a = {}) {
  return r({ __tauriModule: "Fs", message: { cmd: "copyFile", source: t, destination: e, options: a } });
}
async function Ne(t, e = {}) {
  return r({ __tauriModule: "Fs", message: { cmd: "removeFile", path: t, options: e } });
}
async function Ie(t, e, a = {}) {
  return r({ __tauriModule: "Fs", message: { cmd: "renameFile", oldPath: t, newPath: e, options: a } });
}
async function ke(t, e = {}) {
  return r({ __tauriModule: "Fs", message: { cmd: "exists", path: t, options: e } });
}
var Ue = {};
l(Ue, { BaseDirectory: () => f, appCacheDir: () => Ve, appConfigDir: () => B, appDataDir: () => qe, appDir: () => je, appLocalDataDir: () => He, appLogDir: () => G, audioDir: () => Be, basename: () => ft, cacheDir: () => Ge, configDir: () => Je, dataDir: () => Ye, delimiter: () => mt, desktopDir: () => Ke, dirname: () => _t, documentDir: () => Qe, downloadDir: () => Ze, executableDir: () => Xe, extname: () => gt, fontDir: () => et, homeDir: () => tt, isAbsolute: () => wt, join: () => pt, localDataDir: () => at, logDir: () => ct, normalize: () => yt, pictureDir: () => rt, publicDir: () => nt, resolve: () => ht, resolveResource: () => st, resourceDir: () => it, runtimeDir: () => ot, sep: () => dt, templateDir: () => lt, videoDir: () => ut });
async function je() {
  return B();
}
async function B() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 21 } });
}
async function qe() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 22 } });
}
async function He() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 23 } });
}
async function Ve() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 24 } });
}
async function Be() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 1 } });
}
async function Ge() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 2 } });
}
async function Je() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 3 } });
}
async function Ye() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 4 } });
}
async function Ke() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 6 } });
}
async function Qe() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 7 } });
}
async function Ze() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 8 } });
}
async function Xe() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 9 } });
}
async function et() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 10 } });
}
async function tt() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 11 } });
}
async function at() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 5 } });
}
async function rt() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 12 } });
}
async function nt() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 13 } });
}
async function it() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 17 } });
}
async function st(t) {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: t, directory: 17 } });
}
async function ot() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 14 } });
}
async function lt() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 15 } });
}
async function ut() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 16 } });
}
async function ct() {
  return G();
}
async function G() {
  return r({ __tauriModule: "Path", message: { cmd: "resolvePath", path: "", directory: 25 } });
}
var dt = A() ? "\\" : "/", mt = A() ? ";" : ":";
async function ht(...t) {
  return r({ __tauriModule: "Path", message: { cmd: "resolve", paths: t } });
}
async function yt(t) {
  return r({ __tauriModule: "Path", message: { cmd: "normalize", path: t } });
}
async function pt(...t) {
  return r({ __tauriModule: "Path", message: { cmd: "join", paths: t } });
}
async function _t(t) {
  return r({ __tauriModule: "Path", message: { cmd: "dirname", path: t } });
}
async function gt(t) {
  return r({ __tauriModule: "Path", message: { cmd: "extname", path: t } });
}
async function ft(t, e) {
  return r({ __tauriModule: "Path", message: { cmd: "basename", path: t, ext: e } });
}
async function wt(t) {
  return r({ __tauriModule: "Path", message: { cmd: "isAbsolute", path: t } });
}
var bt = {};
l(bt, { exit: () => Mt, relaunch: () => vt });
async function Mt(t = 0) {
  return r({ __tauriModule: "Process", message: { cmd: "exit", exitCode: t } });
}
async function vt() {
  return r({ __tauriModule: "Process", message: { cmd: "relaunch" } });
}
var Pt = {};
l(Pt, { Child: () => J, Command: () => Y, EventEmitter: () => p, open: () => $t });
async function Dt(t, e, a = [], n) {
  return typeof a == "object" && Object.freeze(a), r({ __tauriModule: "Shell", message: { cmd: "execute", program: e, args: a, options: n, onEventFn: c(t) } });
}
var p = class {
  constructor() {
    this.eventListeners = /* @__PURE__ */ Object.create(null);
  }
  addListener(e, a) {
    return this.on(e, a);
  }
  removeListener(e, a) {
    return this.off(e, a);
  }
  on(e, a) {
    return e in this.eventListeners ? this.eventListeners[e].push(a) : this.eventListeners[e] = [a], this;
  }
  once(e, a) {
    let n = (...i) => {
      this.removeListener(e, n), a(...i);
    };
    return this.addListener(e, n);
  }
  off(e, a) {
    return e in this.eventListeners && (this.eventListeners[e] = this.eventListeners[e].filter((n) => n !== a)), this;
  }
  removeAllListeners(e) {
    return e ? delete this.eventListeners[e] : this.eventListeners = /* @__PURE__ */ Object.create(null), this;
  }
  emit(e, ...a) {
    if (e in this.eventListeners) {
      let n = this.eventListeners[e];
      for (let i of n)
        i(...a);
      return !0;
    }
    return !1;
  }
  listenerCount(e) {
    return e in this.eventListeners ? this.eventListeners[e].length : 0;
  }
  prependListener(e, a) {
    return e in this.eventListeners ? this.eventListeners[e].unshift(a) : this.eventListeners[e] = [a], this;
  }
  prependOnceListener(e, a) {
    let n = (...i) => {
      this.removeListener(e, n), a(...i);
    };
    return this.prependListener(e, n);
  }
}, J = class {
  constructor(e) {
    this.pid = e;
  }
  async write(e) {
    return r({ __tauriModule: "Shell", message: { cmd: "stdinWrite", pid: this.pid, buffer: typeof e == "string" ? e : Array.from(e) } });
  }
  async kill() {
    return r({ __tauriModule: "Shell", message: { cmd: "killChild", pid: this.pid } });
  }
}, Y = class extends p {
  constructor(e, a = [], n) {
    super(), this.stdout = new p(), this.stderr = new p(), this.program = e, this.args = typeof a == "string" ? [a] : a, this.options = n != null ? n : {};
  }
  static sidecar(e, a = [], n) {
    let i = new Y(e, a, n);
    return i.options.sidecar = !0, i;
  }
  async spawn() {
    return Dt((e) => {
      switch (e.event) {
        case "Error":
          this.emit("error", e.payload);
          break;
        case "Terminated":
          this.emit("close", e.payload);
          break;
        case "Stdout":
          this.stdout.emit("data", e.payload);
          break;
        case "Stderr":
          this.stderr.emit("data", e.payload);
          break;
      }
    }, this.program, this.args, this.options).then((e) => new J(e));
  }
  async execute() {
    return new Promise((e, a) => {
      this.on("error", a);
      let n = [], i = [];
      this.stdout.on("data", (s) => {
        n.push(s);
      }), this.stderr.on("data", (s) => {
        i.push(s);
      }), this.on("close", (s) => {
        e({ code: s.code, signal: s.signal, stdout: n.join(`
`), stderr: i.join(`
`) });
      }), this.spawn().catch(a);
    });
  }
};
async function $t(t, e) {
  return r({ __tauriModule: "Shell", message: { cmd: "open", path: t, with: e } });
}
var Wt = {};
l(Wt, { getName: () => Lt, getTauriVersion: () => Tt, getVersion: () => At, hide: () => Ct, show: () => Et });
async function At() {
  return r({ __tauriModule: "App", message: { cmd: "getAppVersion" } });
}
async function Lt() {
  return r({ __tauriModule: "App", message: { cmd: "getAppName" } });
}
async function Tt() {
  return r({ __tauriModule: "App", message: { cmd: "getTauriVersion" } });
}
async function Et() {
  return r({ __tauriModule: "App", message: { cmd: "show" } });
}
async function Ct() {
  return r({ __tauriModule: "App", message: { cmd: "hide" } });
}
var St = {};
l(St, { getMatches: () => zt });
async function zt() {
  return r({ __tauriModule: "Cli", message: { cmd: "cliMatches" } });
}
var Ot = {};
l(Ot, { readText: () => Ft, writeText: () => xt });
async function xt(t) {
  return r({ __tauriModule: "Clipboard", message: { cmd: "writeText", data: t } });
}
async function Ft() {
  return r({ __tauriModule: "Clipboard", message: { cmd: "readText", data: null } });
}
var Rt = {};
l(Rt, { ask: () => Ut, confirm: () => jt, message: () => kt, open: () => Nt, save: () => It });
async function Nt(t = {}) {
  return typeof t == "object" && Object.freeze(t), r({ __tauriModule: "Dialog", message: { cmd: "openDialog", options: t } });
}
async function It(t = {}) {
  return typeof t == "object" && Object.freeze(t), r({ __tauriModule: "Dialog", message: { cmd: "saveDialog", options: t } });
}
async function kt(t, e) {
  var n, i;
  let a = typeof e == "string" ? { title: e } : e;
  return r({ __tauriModule: "Dialog", message: { cmd: "messageDialog", message: t.toString(), title: (n = a == null ? void 0 : a.title) == null ? void 0 : n.toString(), type: a == null ? void 0 : a.type, buttonLabel: (i = a == null ? void 0 : a.okLabel) == null ? void 0 : i.toString() } });
}
async function Ut(t, e) {
  var n, i, s, o, h;
  let a = typeof e == "string" ? { title: e } : e;
  return r({ __tauriModule: "Dialog", message: { cmd: "askDialog", message: t.toString(), title: (n = a == null ? void 0 : a.title) == null ? void 0 : n.toString(), type: a == null ? void 0 : a.type, buttonLabels: [(s = (i = a == null ? void 0 : a.okLabel) == null ? void 0 : i.toString()) != null ? s : "Yes", (h = (o = a == null ? void 0 : a.cancelLabel) == null ? void 0 : o.toString()) != null ? h : "No"] } });
}
async function jt(t, e) {
  var n, i, s, o, h;
  let a = typeof e == "string" ? { title: e } : e;
  return r({ __tauriModule: "Dialog", message: { cmd: "confirmDialog", message: t.toString(), title: (n = a == null ? void 0 : a.title) == null ? void 0 : n.toString(), type: a == null ? void 0 : a.type, buttonLabels: [(s = (i = a == null ? void 0 : a.okLabel) == null ? void 0 : i.toString()) != null ? s : "Ok", (h = (o = a == null ? void 0 : a.cancelLabel) == null ? void 0 : o.toString()) != null ? h : "Cancel"] } });
}
var qt = {};
l(qt, { isRegistered: () => Bt, register: () => Ht, registerAll: () => Vt, unregister: () => Gt, unregisterAll: () => Jt });
async function Ht(t, e) {
  return r({ __tauriModule: "GlobalShortcut", message: { cmd: "register", shortcut: t, handler: c(e) } });
}
async function Vt(t, e) {
  return r({ __tauriModule: "GlobalShortcut", message: { cmd: "registerAll", shortcuts: t, handler: c(e) } });
}
async function Bt(t) {
  return r({ __tauriModule: "GlobalShortcut", message: { cmd: "isRegistered", shortcut: t } });
}
async function Gt(t) {
  return r({ __tauriModule: "GlobalShortcut", message: { cmd: "unregister", shortcut: t } });
}
async function Jt() {
  return r({ __tauriModule: "GlobalShortcut", message: { cmd: "unregisterAll" } });
}
var Yt = {};
l(Yt, { Body: () => y, Client: () => Z, Response: () => Q, ResponseType: () => K, fetch: () => Kt, getClient: () => X });
var K = ((t) => (t[t.JSON = 1] = "JSON", t[t.Text = 2] = "Text", t[t.Binary = 3] = "Binary", t))(K || {}), y = class {
  constructor(t, e) {
    this.type = t, this.payload = e;
  }
  static form(t) {
    let e = {}, a = (n, i) => {
      if (i !== null) {
        let s;
        typeof i == "string" ? s = i : i instanceof Uint8Array || Array.isArray(i) ? s = Array.from(i) : i instanceof File ? s = { file: i.name, mime: i.type, fileName: i.name } : typeof i.file == "string" ? s = { file: i.file, mime: i.mime, fileName: i.fileName } : s = { file: Array.from(i.file), mime: i.mime, fileName: i.fileName }, e[String(n)] = s;
      }
    };
    if (t instanceof FormData)
      for (let [n, i] of t)
        a(n, i);
    else
      for (let [n, i] of Object.entries(t))
        a(n, i);
    return new y("Form", e);
  }
  static json(t) {
    return new y("Json", t);
  }
  static text(t) {
    return new y("Text", t);
  }
  static bytes(t) {
    return new y("Bytes", Array.from(t instanceof ArrayBuffer ? new Uint8Array(t) : t));
  }
}, Q = class {
  constructor(t) {
    this.url = t.url, this.status = t.status, this.ok = this.status >= 200 && this.status < 300, this.headers = t.headers, this.rawHeaders = t.rawHeaders, this.data = t.data;
  }
}, Z = class {
  constructor(t) {
    this.id = t;
  }
  async drop() {
    return r({ __tauriModule: "Http", message: { cmd: "dropClient", client: this.id } });
  }
  async request(t) {
    let e = !t.responseType || t.responseType === 1;
    return e && (t.responseType = 2), r({ __tauriModule: "Http", message: { cmd: "httpRequest", client: this.id, options: t } }).then((a) => {
      let n = new Q(a);
      if (e) {
        try {
          n.data = JSON.parse(n.data);
        } catch (i) {
          if (n.ok && n.data === "")
            n.data = {};
          else if (n.ok)
            throw Error(`Failed to parse response \`${n.data}\` as JSON: ${i};
              try setting the \`responseType\` option to \`ResponseType.Text\` or \`ResponseType.Binary\` if the API does not return a JSON response.`);
        }
        return n;
      }
      return n;
    });
  }
  async get(t, e) {
    return this.request(u({ method: "GET", url: t }, e));
  }
  async post(t, e, a) {
    return this.request(u({ method: "POST", url: t, body: e }, a));
  }
  async put(t, e, a) {
    return this.request(u({ method: "PUT", url: t, body: e }, a));
  }
  async patch(t, e) {
    return this.request(u({ method: "PATCH", url: t }, e));
  }
  async delete(t, e) {
    return this.request(u({ method: "DELETE", url: t }, e));
  }
};
async function X(t) {
  return r({ __tauriModule: "Http", message: { cmd: "createClient", options: t } }).then((e) => new Z(e));
}
var w = null;
async function Kt(t, e) {
  var a;
  return w === null && (w = await X()), w.request(u({ url: t, method: (a = e == null ? void 0 : e.method) != null ? a : "GET" }, e));
}
var Qt = v;
window.__HC_LAUNCHER_API__ = {
  notify: async (t) => {
    await Qt("notify", { notification: t });
  }
};
