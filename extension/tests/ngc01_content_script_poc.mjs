/**
 * NGC-01 regression test — drives the REAL extension/src/content.ts through
 * the same message interface popup.ts uses.
 *
 * The original finding: findPSBT scraped #request-type and #channel-amount
 * as innerHTML from the requesting page, and those page-authored strings
 * became the "Operation Type" and "Amount" fields on the approval screen.
 *
 * After the fix, findPSBT returns ONLY the PSBT: the page has no channel to
 * put text on the approval screen, which derives everything it displays from
 * the PSBT itself.
 *
 * The pastePSBT half of the script is unchanged by this fix; the assertions
 * below pin its current behaviour so a later change to it is deliberate.
 *
 * Run: node extension/tests/ngc01_content_script_poc.mjs
 */
import { execFileSync } from "node:child_process";
import { mkdtempSync, existsSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import vm from "node:vm";
import assert from "node:assert/strict";

const here = dirname(fileURLToPath(import.meta.url));
const source = join(here, "..", "src", "content.ts");

// ---------------------------------------------------------------- compile ----
const out = mkdtempSync(join(tmpdir(), "ngc01-"));
try {
  // `browser` resolves via @types/firefox-webext-browser, which is not
  // installed here; tsc still emits. Exit code is therefore not checked.
  execFileSync(
    "npx",
    ["--no-install", "tsc", source, "--outDir", out,
     "--target", "es2020", "--module", "commonjs", "--skipLibCheck"],
    { stdio: "pipe" },
  );
} catch { /* type errors are expected without the ambient types */ }

const compiled = join(out, "content.js");
assert.ok(existsSync(compiled), "content.ts failed to compile");
console.log(`compiled ${source} -> ${compiled}`);

// ------------------------------------------------------- the hostile page ----
// The page still offers the elements the old content.ts used to scrape.
const ATTACKER_PSBT = "cHNidP8BAIcBAAAAAvTk...ATTACKER-CHOSEN-OUTPUTS...AAA=";

const clicked = [];
const page = {
  "psbt-to-sign":  { value: ATTACKER_PSBT },
  // Page-authored free text. Must never reach the extension again.
  "request-type":  { innerHTML: "Channel Open" },
  "channel-amount": { innerHTML: "0.001" },
  "psbt-to-paste": { value: "", dispatchEvent: () => true },
  "approve-button": {
    focus: () => clicked.push("focus"),
    click: () => clicked.push("click"),
  },
};

let listener = null;
const sandbox = {
  console,
  setTimeout,
  Event: class { constructor(type) { this.type = type; } },
  document: { getElementById: (id) => page[id] ?? null },
  browser: { runtime: { onMessage: { addListener: (fn) => { listener = fn; } } } },
};

vm.createContext(sandbox);
vm.runInContext(readFileSync(compiled, "utf8"), sandbox, { filename: "content.js" });
assert.ok(typeof listener === "function", "content.ts registered no message listener");

// --------------------------------------------------- 1. what gets scraped ----
// popup.ts:18 sends exactly this message.
let scraped;
listener({ type: "findPSBT" }, undefined, (r) => { scraped = r; });
console.log("\nfindPSBT returned:", scraped);

assert.equal(scraped.psbt, ATTACKER_PSBT, "the PSBT is taken from the page");
assert.equal(scraped.request_type, undefined,
  "page-authored #request-type must not be scraped");
assert.equal(scraped.amount, undefined,
  "page-authored #channel-amount must not be scraped");
assert.deepEqual(Object.keys(scraped), ["psbt"],
  "findPSBT returns the PSBT and nothing else");
console.log("-> only the PSBT crosses into the extension;");
console.log("   the page has no channel onto the approval screen");

// ------------------------------- 2. signed material returns to that page ----
const SIGNED = "cHNidP8B-SIGNED-BY-THE-USERS-KEY-AAA=";
listener({ type: "pastePSBT", psbt: SIGNED }, undefined, () => {});

assert.equal(page["psbt-to-paste"].value, SIGNED, "signed PSBT handed to the page");
console.log("\npastePSBT wrote the signed PSBT into the page's own input");

// Unchanged by this fix: content.ts focuses and clicks the page's
// #approve-button on a timer. Pinned here so a later change is deliberate.
assert.deepEqual(clicked, [], "not yet clicked");
await new Promise((r) => setTimeout(r, 700));
assert.deepEqual(clicked, ["focus", "click"],
  "content.ts submits the signed PSBT back to the page");
console.log("-> submit-back behaviour unchanged by this fix");

console.log("\nPASS: content.ts no longer sources any displayed value from the");
console.log("      requesting page; the approval screen is PSBT-derived only.");
