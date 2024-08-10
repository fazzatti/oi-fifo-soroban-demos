import { readFile } from "fs/promises";

export async function loadWasmFile(wasmFilePath: string) {
  try {
    const buffer = await readFile(wasmFilePath);
    return buffer;
  } catch (error) {
    console.error(`Error reading the WASM file: ${error}`);
    throw error;
  }
}
