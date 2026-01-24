<template>
    <table>
        <thead>
            <tr>
                <th>Lock</th>
                <th>Server Name</th>
                <th>Ping</th>
                <th>Players</th>
                <th>Version</th>
                <th>Gamemode</th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="server in props.data" :key="hashServer(server.server)">
                <td>{{ server.password }}</td>
                <td>{{ server.servername }}</td>
                <td>{{ toMillis(server.elapsed) }}</td>
                <td>{{ server.players }}/{{ server.maxplayers }}</td>
                <td>{{ server.version }}</td>
                <td>{{ server.gamemode }}</td>
            </tr>
        </tbody>
    </table>
</template>
<script setup lang="ts">
import { ServerInfo } from '../types';
import { hashServer, toMillis } from '../utils';

const props = defineProps({
    data: {
        type: Array as () => ServerInfo[],
        default: () => [],
    },
});
</script>
