<script setup lang="ts">
import { computed } from 'vue';
import { greet } from '../bridge';
import { invoke } from '@tauri-apps/api/core';

const hello = computed(async () => {
    return invoke("greet", { name: "World"});
})
</script>

<template>
    <h4>{{ hello }}</h4>
</template>
