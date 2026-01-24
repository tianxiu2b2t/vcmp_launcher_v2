<template>
    <!-- <div v-for="value in internetServers">
        <div>{{ JSON.stringify(value) }}</div>
    </div> -->
    <ServerList :data="internetServers"></ServerList>
</template>
<script setup lang="ts">
import { onMounted } from 'vue';
// import { ServerInfo } from '../types';
import { internetServers, refreshInternetServers } from '../constants';
import ServerList from '../components/ServerList.vue';
onMounted(refreshInternetServers);
</script>
