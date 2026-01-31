// src/i18n/index.ts
import { createI18n } from 'vue-i18n';

// 动态导入所有语言文件
const localesModules = import.meta.glob('./locales/*.json', {
    eager: true,
    import: 'default',
});

// 分离通用配置和语言配置
let commonTranslations: Record<string, any> = {};
const languageFiles: Record<string, any> = {};

// 处理导入的文件
for (const path in localesModules) {
    const fileNameMatch = path.match(/\/([^\/]+)\.json$/);
    if (!fileNameMatch) continue;

    const fileName = fileNameMatch[1];

    if (fileName === 'common') {
        commonTranslations = localesModules[path] as Record<string, any>;
    } else {
        languageFiles[fileName] = localesModules[path];
    }
}

// 构建完整的 messages 对象，将 common 合并到每个语言中
const messages: Record<string, any> = {};

Object.entries(languageFiles).forEach(([lang, translations]) => {
    messages[lang] = {
        common: commonTranslations,
        ...translations,
    };
});

// 创建 i18n 实例
export const i18n = createI18n({
    legacy: false, // 使用 Composition API
    locale: 'zh_cn',
    fallbackLocale: 'en',
    messages,
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

// 导出支持的语言列表
export const availableLocales = Object.keys(messages);

// 切换语言
export function setLocale(locale: string) {
    if (messages[locale]) {
        i18n.global.locale.value = locale;
        localStorage.setItem('app-language', locale);
        document.documentElement.lang = locale;
        return true;
    }
    console.warn(`[i18n] Language "${locale}" is not available`);
    return false;
}

// 获取当前语言
export function getLocale(): string {
    return i18n.global.locale.value;
}

// 获取语言显示名称
export function getLanguageName(locale: string): string {
    const languageNames: Record<string, string> = {
        en: 'English',
        'zh-cn': '简体中文',
        zh: '中文',
        'zh-tw': '繁體中文',
        ja: '日本語',
        ko: '한국어',
        fr: 'Français',
        de: 'Deutsch',
        es: 'Español',
    };

    return languageNames[locale] || locale;
}

// 检查翻译键是否存在
export function hasTranslation(key: string, locale?: string): boolean {
    const targetLocale = locale || getLocale();
    const msg = messages[targetLocale];

    if (!msg) return false;

    const keys = key.split('.');
    let current = msg;

    for (const k of keys) {
        if (current && typeof current === 'object' && k in current) {
            current = current[k];
        } else {
            return false;
        }
    }

    return typeof current === 'string';
}

// 开发环境：检查翻译完整性
// if (import.meta.env.DEV) {
//     setTimeout(() => {
//         console.group('[i18n] Translation Status');
//         console.log('Available locales:', availableLocales);

//         // 检查每个语言是否有所有键
//         const allKeys = new Set<string>();

//         // 收集所有语言的键
//         Object.values(messages).forEach((msg) => {
//             const collectKeys = (obj: any, prefix = '') => {
//                 for (const key in obj) {
//                     if (key === 'common') continue; // 跳过 common，因为是共享的

//                     const fullKey = prefix ? `${prefix}.${key}` : key;
//                     if (typeof obj[key] === 'object' && obj[key] !== null) {
//                         collectKeys(obj[key], fullKey);
//                     } else if (typeof obj[key] === 'string') {
//                         allKeys.add(fullKey);
//                     }
//                 }
//             };
//             collectKeys(msg);
//         });

//         console.log('Total unique keys:', allKeys.size);

//         // 检查每个语言缺失的键
//         availableLocales.forEach((locale) => {
//             const msg = messages[locale];
//             const missingKeys: string[] = [];

//             allKeys.forEach((key) => {
//                 if (!hasTranslation(key, locale)) {
//                     missingKeys.push(key);
//                 }
//             });

//             if (missingKeys.length > 0) {
//                 console.warn(
//                     `"${locale}" is missing ${missingKeys.length} keys:`,
//                     missingKeys,
//                 );
//             } else {
//                 console.log(`"${locale}": Complete`);
//             }
//         });

//         console.groupEnd();
//     }, 1000);
// }

export default i18n;
