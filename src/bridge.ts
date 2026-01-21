import { invoke } from "@tauri-apps/api/core";
import { GameConfig, InternetConfig } from "./types";

export async function greet(name: string) {
    return await invoke("greet", { name: name })
}

export async function get_game_config(): Promise<GameConfig> {
    return await invoke("get_game_config")
}

export async function set_game_config(data?: GameConfig): Promise<boolean> {
    if (!data) return true;
    return await invoke("set_game_config", { data: data })
}

export async function get_internet_config(): Promise<InternetConfig> {
    return await invoke("get_internet")
}

export async function set_internet_config(data?: GameConfig): Promise<boolean> {
    if (!data) return true;
    return await invoke("set_internet", { data: data })
}