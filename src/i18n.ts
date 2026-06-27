// src/i18n/index.ts
import { createI18n } from 'vue-i18n';

// 动态导入所有语言文件
const localesModules: Record<string, any> = import.meta.glob(
    './locales/*.json',
    {
        eager: true,
        import: 'default',
    },
);
// get common.json
const common =
    Object.entries(localesModules).find(([key]) =>
        key.endsWith('common.json'),
    )?.[1] || {};

// replace ${common.xxx} with value
// or ${common.xxx.yyy} or ${common.xxx.yyy.zzz} or more
function replaceTemplate(template: string) {
    return template.replace(/\$\{common\.(\w+(\.\w+)*)\}/g, (_, key) => {
        const keys = key.split('.');
        let value = common;
        for (const k of keys) {
            value = value[k];
        }
        return value;
    });
}

function iterReplace(obj: any) {
    if (typeof obj === 'object' && obj !== null) {
        Object.keys(obj).forEach((key) => {
            obj[key] = iterReplace(obj[key]);
        });
    } else if (typeof obj === 'string') {
        obj = replaceTemplate(obj);
    }
    return obj;
}

const messages: Record<string, any> = {};
Object.entries(localesModules).forEach(([key, value]) => {
    const locale = key.replace('./locales/', '').replace('.json', '');
    if (locale === 'common') {
        return;
    }
    messages[locale] = {
        common,
        ...iterReplace(value),
    };
});

// 创建 i18n 实例
export const i18n = createI18n({
    legacy: false, // 使用 Composition API
    locale: 'zh_cn',
    fallbackLocale: 'en',
    messages: messages,
    silentTranslationWarn: import.meta.env.PROD,
    missingWarn: import.meta.env.DEV,
    fallbackWarn: import.meta.env.DEV,
    missing: (locale, key) => {
        // 开发环境显示缺失键警告
        if (import.meta.env.DEV) {
            console.warn(
                `[i18n] Missing translation for "${key}" in locale "${locale}"`,
            );
        }
        return key;
    },
});

export default i18n;
