<template></template>
<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { fetchInternetServers, pingServer } from '../bridge';
import { ServerInfo } from '../types';
const servers = ref<ServerInfo[]>([]);
onMounted(async () => {
    const internetServers = await fetchInternetServers();
    await Promise.all(
        internetServers.map(async (v) => {
            const server = await pingServer(v);
            servers.value.push(server);
        }),
    );
});
</script>
