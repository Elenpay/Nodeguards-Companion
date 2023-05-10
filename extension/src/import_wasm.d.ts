export const import_wasm: () => Promise<typeof import('../pkg/extension')>;

interface CustomWindow extends Window {
    pastePSBT(psbt: string): Promise<void>;
    savePassword(password: string): Promise<void>;
    getPassword(): Promise<{ password: string; }>;
}
