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
        },
        zh: {
            tabs: {
                home: '首页',
                internet: '网络',
                settings: '设置',
            },
        },
    },
});
