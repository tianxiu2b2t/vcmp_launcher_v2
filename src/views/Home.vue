<script setup lang="ts">
import { computed } from 'vue';
import { greet } from '../bridge';

const hello = computed(async () => {
    return await greet("cnm");
})
</script>

<template>
    <h4>{{ hello }}</h4>
</template>
