import { invoke } from "@tauri-apps/api";

declare global {
  interface Window {
    __HC_LAUNCHER_API__: {
      notify: (notification: string) => void;
    };
  }
}

window.__HC_LAUNCHER_API__.notify = async (notification: string) => {
  await invoke("notify", { notification });
};
