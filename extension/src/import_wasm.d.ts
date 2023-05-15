export const import_wasm: () => Promise<typeof import('../pkg/extension')>;

interface CustomWindow extends Window {
    sessionExists(): bool;
    pastePSBT(psbt: string): Promise<void>;
    savePassword(password: string): Promise<void>;
    getPassword(): Promise<{ password: string; }>;
}
