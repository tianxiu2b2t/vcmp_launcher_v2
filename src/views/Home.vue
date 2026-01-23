<script setup lang="ts">
import { computedAsync } from '@vueuse/core';
import { get_config } from '../bridge';
// import { greet } from '../bridge';

const config = computedAsync(async () => {
    // const value = await greet("name");
    return get_config();
})
</script>

<template>
    <h4>{{ JSON.stringify(config) }}</h4>
</template>
