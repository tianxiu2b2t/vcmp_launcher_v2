<script setup lang="ts">
import { computedAsync } from '@vueuse/core';
import { greet } from '../bridge';
import { invoke } from '@tauri-apps/api/core';

const hello = computedAsync(async () => {
    return await greet("name")
})
</script>

<template>
    <h4>{{ hello }}</h4>
</template>
