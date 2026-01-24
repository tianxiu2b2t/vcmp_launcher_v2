import { invoke } from '@tauri-apps/api/core';
import { Config, Server, ServerInfo } from './types';

// export async function greet(name: string) {
//     return await invoke("greet", { name: name })
// }

// export async function get_game_config(): Promise<GameConfig> {
//     return await invoke("get_game_config")
// }

// export async function set_game_config(data?: GameConfig): Promise<boolean> {
//     if (!data) return true;
//     return await invoke("set_game_config", { data: data })
// }

// export async function get_internet_config(): Promise<InternetConfig> {
//     return await invoke("get_internet")
// }

// export async function set_internet_config(data?: GameConfig): Promise<boolean> {
//     if (!data) return true;
//     return await invoke("set_internet", { data: data })
// }

export async function getConfig(): Promise<Config> {
    return await invoke('get_config');
}

export async function setConfig(config: Config) {
    return await invoke('set_config', { config });
}

export async function fetchInternetServer(): Promise<Server[]> {
    return await invoke('fetch_internet_server');
}

export async function pingServer(server: Server): Promise<ServerInfo> {
    return await invoke('ping_server', { server });
}
