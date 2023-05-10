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

const clearPassword = async () => {
    console.log("clearing password");
    await browser.storage.session.clear();
};

window.savePassword = async (password: string) => {
    await browser.storage.session.set({ password, expiration: Date.now() + 1000 * 60 * 5 });
};

window.getPassword = async () => {
    let password_data = await browser.storage.session.get("password");
    let expiration_data = await browser.storage.session.get("expiration");
    console.log(expiration_data);
    if (!expiration_data?.expiration || expiration_data.expiration < Date.now()) {
        await clearPassword();
        return "";
    }
    console.log(password_data);
    return password_data.password;
};