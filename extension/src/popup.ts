declare var window: CustomWindow;
declare var chrome: any;
declare var browser: any;

import { CustomWindow, import_wasm } from './import_wasm';

async function sendMessage(message: any) {
    const [tab] = await browser.tabs.query({ active: true, lastFocusedWindow: true });
    if (!tab?.id) {
        return {};
    }
    return await browser.tabs.sendMessage(tab.id, message, {});
}

(async () => {
    let wasm = await import_wasm();

    let psbtData = await sendMessage({ type: "findPSBT" });
    if (psbtData?.psbt) {
        wasm.approve_psbt(psbtData);
    }
})();

window.pastePSBT = async function pastePSBT(psbt: string) {
    await sendMessage({ type: "pastePSBT", psbt });
};

declare namespace browser.storage.session {
    export interface BrowserSession {
        clear(): Promise<void>;
        set(data: { password: string; }): Promise<void>;
        get(key: string): Promise<{ password: string; }>;
    }
}

window.sessionExists = () => !!browser?.storage?.session;

const clearPassword = async () => {
    if (!browser) {
        return;
    }
    await browser?.storage?.session?.clear?.();
};

window.savePassword = async (password: string) => {
    if (!browser) {
        return;
    }
    await browser?.storage?.session?.set?.({ password, expiration: Date.now() + 1000 * 60 * 5 });
};

window.getPassword = async () => {
    if (!browser) {
        return "";
    }
    let password_data = await browser?.storage?.session?.get?.("password");
    let expiration_data = await browser?.storage?.session?.get?.("expiration");
    if (!expiration_data?.expiration || expiration_data.expiration < Date.now()) {
        await clearPassword();
        return "";
    }
    return password_data.password;
};

window.openOptionsPage = () => {
    browser.runtime.openOptionsPage();
    window.close();
};
