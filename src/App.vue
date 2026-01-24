<template>
    <Progressbar />
    <Switchbar :data="tabs" :model-value="active"></Switchbar>
    <main>
        <RouterView></RouterView>
    </main>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import Progressbar from './components/Progressbar.vue';
import Switchbar from './components/Switchbar.vue';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
const tabs = [
    { text: t('tabs.home'), path: '/' },
    {
        text: t('tabs.internet'),
        path: '/internet',
    },
    {
        text: t('tabs.settings'),
        path: '/settings',
    },
    {
        text: 'Debug',
        path: '/debug',
    },
];
const active = ref<number>(0);
</script>

<style>
:root.dark {
    --bg-color: #1f1f1f;
    --main-color: #f4d1b4;
    --dark-color: rgba(0, 0, 0, 0.7);
    --color: rgba(255, 255, 255, 0.7);
    color: var(--color);
}
:root {
    --bg-color: #f5f5f5;
    --main-color: #0fc6c2;
    --dark-color: rgba(255, 255, 255, 0.7);
    --color: rgba(0, 0, 0, 0.7);
    color: var(--color);
}
body {
    background-color: var(--bg-color);
    width: 100vw;
    height: 100vh;
    margin: 0;
    padding: 0;
    overflow: auto;
}
</style>
