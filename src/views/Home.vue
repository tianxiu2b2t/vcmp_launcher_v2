<script setup lang="ts">
import { computedAsync } from '@vueuse/core';
import { greet } from '../bridge';

const hello = computedAsync(async () => {
    const value = await greet("name");
    console.log(value)
    return value;
})
</script>

<template>
    <h4>{{ hello }}</h4>
</template>
