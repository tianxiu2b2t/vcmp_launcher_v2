import { computed, ref, toRef, watch } from 'vue';
import { createRouter, createWebHistory } from 'vue-router';
import {
    errorMessage,
    fetchInternetServers,
    getConfig,
    pingServer,
    setConfig,
} from './bridge';
import { ServerInfo } from './types';
import { hashServer } from './utils';

export const routes = [
    {
        path: '/',
        name: 'Home',
        component: () => import('./views/Home.vue'),
    },
    {
        path: '/internet',
        name: 'Internet',
        component: () => import('./views/Internet.vue'),
    },
    {
        path: '/settings',
        name: 'Settings',
        component: () => import('./views/Settings.vue'),
    },
    {
        path: '/about',
        name: 'About',
        component: () => import('./views/About.vue'),
    },
    {
        path: '/debug',
        name: 'Debug',
        component: () => import('./views/Debug.vue'),
    },
];

export const router = createRouter({
    history: createWebHistory(),
    routes,
});

export const systemDarkMode = computed(() => {
    return window.matchMedia('(prefers-color-scheme: dark)').matches;
});

export const progressbar = ref(0);

export const config = toRef(await getConfig());
watch(
    config,
    async (newConfig) => {
        console.log('config', newConfig);
        setConfig(newConfig);
    },
    { deep: true },
);

(() => {
    if (systemDarkMode.value) {
        document.documentElement.classList.add('dark');
    }
})();

export const internetServers = ref<ServerInfo[]>([]);
/** 服务器列表加载中（拉列表 + 批量 ping） */
export const internetServersLoading = ref(false);
/** 拉取服务器列表失败时的错误信息（网络不可用、镜像站异常等），为空表示没出错 */
export const internetServersError = ref('');

export async function refreshInternetServers() {
    if (internetServersLoading.value) return;
    internetServersLoading.value = true;
    internetServersError.value = '';
    try {
        const servers = await fetchInternetServers();
        // 单个服务器 ping 失败不应影响整个列表，因此逐个捕获
        const results = await Promise.all(
            servers.map(async (v) => {
                try {
                    return await pingServer(v);
                } catch (error) {
                    console.warn(
                        `[internet] ping ${hashServer(v)} failed:`,
                        errorMessage(error),
                    );
                    return null;
                }
            }),
        );
        for (const server of results) {
            if (!server) continue;
            const idx = internetServers.value.findIndex(
                (s) => hashServer(server.server) === hashServer(s.server),
            );
            if (idx !== -1) {
                internetServers.value[idx] = server;
                continue;
            }
            internetServers.value.push(server);
        }
    } catch (error) {
        internetServersError.value = errorMessage(error);
        console.error('[internet] fetch servers failed:', error);
    } finally {
        internetServersLoading.value = false;
    }
}
