interface OperationRequestData {
    psbt?: string,
    request_type?: string,
    amount?: string
}

function findPSBT() {
    let psbtField = document.getElementById("psbt-to-sign") as HTMLInputElement | null;
    let requestTypeField = document.getElementById("request-type");
    let channelAmountField = document.getElementById("channel-amount");
    let data: OperationRequestData = {}
    if (psbtField?.value) {
        data["psbt"] = psbtField.value;
    }
    if (requestTypeField?.innerHTML) {
        data["request_type"] = requestTypeField.innerHTML;
    }
    if (channelAmountField?.innerHTML) {
        data["amount"] = channelAmountField.innerHTML;
    }
    return data;
}

function pastePSBT(signedPsbt: string) {
    let psbtField = document.getElementById("psbt-to-paste") as HTMLInputElement | null;
    if (psbtField) {
        psbtField.innerHTML = signedPsbt;
    }
}

browser.runtime.onMessage.addListener((message, _, sendResponse) => {
    switch (message?.type) {
        case "findPSBT":
            sendResponse(findPSBT());
            break;
        case "pastePSBT":
            sendResponse(pastePSBT(message.psbt))
            break;
    }
});
