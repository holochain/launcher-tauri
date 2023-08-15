declare global {
  interface Window {
    __HC_LAUNCHER_API__: {
      notify: (notification: string) => void;
    };
  }
}

export async function notify(notification: string) {
  window.__HC_LAUNCHER_API__.notify(notification);
}
