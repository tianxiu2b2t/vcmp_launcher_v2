export interface GameConfig {
    username: string;
    game_dir: string;
}

export interface InternetConfig {
    master_url: string;
    update_url: string;
    password?: string;
}