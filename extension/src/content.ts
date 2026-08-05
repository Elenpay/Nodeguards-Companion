interface OperationRequestData {
    psbt?: string,
}

// Only the PSBT itself is taken from the page. Anything else a page could
// supply (operation type, amounts, ...) is unverifiable and must never be
// shown on the approval screen (NGC-01) — the UI derives everything it
// displays from the PSBT.
function findPSBT() {
    let psbtField = document.getElementById("psbt-to-sign") as HTMLInputElement | null;
    let data: OperationRequestData = {};
    if (psbtField?.value) {
        data["psbt"] = psbtField.value;
    }
    return data;
}

function pastePSBT(signedPsbt: string) {
    let psbtField = document.getElementById("psbt-to-paste") as HTMLInputElement | null;
    let approveButton = document.getElementById("approve-button") as HTMLButtonElement | null;

    if (psbtField) {
        psbtField.value = signedPsbt;
        psbtField.dispatchEvent(new Event("change"));
    }
    setTimeout(() => {
        if (approveButton) {
            approveButton.focus();
            approveButton.click();
        }
    }, 500);
}

browser.runtime.onMessage.addListener((message, _, sendResponse) => {
    switch (message?.type) {
        case "findPSBT":
            sendResponse(findPSBT());
            break;
        case "pastePSBT":
            sendResponse(pastePSBT(message.psbt));
            break;
    }
});
