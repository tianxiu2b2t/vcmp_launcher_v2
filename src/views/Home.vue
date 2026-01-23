<script setup lang="ts">
import { computedAsync } from '@vueuse/core';
import { get_config } from '../bridge';
// import { greet } from '../bridge';

const hello = computedAsync(async () => {
    // const value = await greet("name");
    // return value;
})
</script>

<template>
    <h4>{{ JSON.stringify(get_config()) }}</h4>
</template>
