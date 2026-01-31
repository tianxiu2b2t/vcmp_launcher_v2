import { computed, ref, toRef, watch } from 'vue';
import { createRouter, createWebHistory } from 'vue-router';
import {
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
export async function refreshInternetServers() {
    const servers = await fetchInternetServers();
    await Promise.all(
        servers.map(async (v) => {
            const server = await pingServer(v);
            const idx = internetServers.value.findIndex(
                (s) => hashServer(v) === hashServer(s.server),
            );
            if (idx !== -1) {
                internetServers.value[idx] = server;
                return;
            }
            internetServers.value.push(server);
        }),
    );
}
