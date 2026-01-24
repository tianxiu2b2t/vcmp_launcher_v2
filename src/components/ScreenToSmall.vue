<template>
    <div class="screen-to-small">
        <h1>Screen is too small</h1>
        <p>Please use a larger screen to view this page</p>
    </div>
</template>

<script setup lang="ts">
import { watch } from 'vue';
import { getWindowSize } from '../assets/componesables/screen';
import type { WindowSize } from '../types';
const props = defineProps({
    size: {
        type: Object as () => WindowSize,
        default: () => ({
            width: 800,
            height: 600,
        }),
    },
    percent: {
        type: Number,
        default: 0.95,
    },
});
// const body = document.querySelector('body');
watch(
    () => getWindowSize(),
    (value) => {
        const { width, height } = value.value;
        const sizeWidth = props.size.width * props.percent;
        const sizeHeight = props.size.height * props.percent;
        console.log(width, height, sizeWidth, sizeHeight);
    },
    {
        immediate: true,
    },
);
</script>

<style scoped>
.screen-to-small {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
}
</style>
