import { invoke } from "@tauri-apps/api/core";

export async function greet(name: string) {
    await invoke("greet", { name: name })
}