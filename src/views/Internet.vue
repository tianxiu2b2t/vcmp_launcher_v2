<template>
    <div class="internet">
        <div class="toolbar">
            <span v-if="internetServers.length" class="count">
                {{ internetServers.length }}
            </span>
            <button
                class="refresh"
                :disabled="internetServersLoading"
                @click="refreshInternetServers"
            >
                {{ $t('internet.refresh') }}
            </button>
        </div>

        <p v-if="internetServersLoading && !internetServers.length" class="state">
            {{ $t('internet.loading') }}
        </p>

        <p v-else-if="internetServersError && !internetServers.length" class="state error">
            {{ $t('internet.error') }}：{{ internetServersError }}
            <br />
            <small>{{ $t('internet.hint') }}</small>
        </p>

        <p v-else-if="!internetServers.length" class="state">
            {{ $t('list.empty') }}
        </p>

        <ServerList v-else :data="internetServers"></ServerList>
    </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import {
    internetServers,
    internetServersError,
    internetServersLoading,
    refreshInternetServers,
} from '../constants';
import ServerList from '../components/ServerList.vue';

onMounted(refreshInternetServers);
</script>

<style scoped>
.toolbar {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 8px;
    margin-bottom: 8px;
}

.count {
    opacity: 0.6;
}

.refresh {
    padding: 4px 12px;
    border-radius: 4px;
    border: 1px solid var(--main-color);
    background-color: transparent;
    color: var(--main-color);
    cursor: pointer;
}

.refresh:disabled {
    opacity: 0.5;
    cursor: default;
}

.state {
    color: var(--color);
}

.state.error {
    color: #e5484d;
}
</style>
