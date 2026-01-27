import { createI18n } from 'vue-i18n';

export const i18n = createI18n({
    locale: 'zh',
    fallbackLocale: 'en',
    messages: {
        en: {
            tabs: {
                home: 'Home',
                internet: 'Internet',
                settings: 'Settings',
            },
            list: {
                servername: 'Server Name',
                gamemode: 'GameMode',
                players: 'Players',
                version: 'Version',
                ping: 'Ping',
            },
        },
        zh: {
            tabs: {
                home: '首页',
                internet: '网络',
                settings: '设置',
            },
            list: {
                servername: '服务器名称',
                gamemode: '游戏模式',
                players: '在线玩家',
                version: '版本',
                ping: '延迟',
            },
        },
    },
});
