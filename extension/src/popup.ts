import { import_wasm } from './import_wasm';

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