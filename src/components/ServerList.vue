<template>
    <table>
        <thead>
            <tr>
                <th></th>
                <th>{{ $t('list.servername') }}</th>
                <th>{{ $t('list.ping') }}</th>
                <th>{{ $t('list.players') }}</th>
                <th>{{ $t('list.version') }}</th>
                <th>{{ $t('list.gamemode') }}</th>
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
