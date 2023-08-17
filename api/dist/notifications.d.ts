declare global {
    interface Window {
        __HC_LAUNCHER_API__: {
            notify: (notification: string) => void;
        };
    }
}
export declare function notify(notification: string): Promise<void>;
