export interface GameConfig {
    username?: string;
    game_dir?: string;
}

export interface InternetConfig {
    master_url?: string;
    update_url?: string;
    password?: string;
}

export interface Config {
    game: GameConfig;
    internet: InternetConfig;
}

export interface Server {
    ip: string;
    port: number;
    official: boolean;
}

export interface ServerInfo {
    server: Server;
    servername: string;
    gamemode: string;
    map: string;
    players: number;
    maxplayers: number;
    password: boolean;
    players_list: string[];
    version: string;
}
