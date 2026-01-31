<template>
    <div
        class="tabs-outer-container"
        :style="`--scale: ${$props.scale}; --floor-scale: ${floorScale}`"
    >
        <div class="tabs-container">
            <button
                v-for="(value, idx) in innerData"
                class="tabs-button"
                :class="{ selected: idx == active }"
                @click="active = idx"
                ref="btns"
            >
                {{ value.text }}
            </button>
        </div>
        <span class="tabs-block" ref="block"></span>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { TabButton } from '../types';
import { router } from '../constants';

const props = defineProps({
    data: {
        type: Array as () => Array<string | TabButton>,
        default: () => [],
    },
    active: {
        type: Number,
        default: 0,
    },
    scale: {
        type: Number,
        default: 1,
    },
});
const floorScale = Math.floor(props.scale);
const innerData: TabButton[] = props.data.map((item) => {
    if (typeof item == 'string') {
        return {
            text: item,
        };
    }
    return item;
});
const active = ref<number>(props.active);
const block = ref<HTMLElement>();
const btns = ref<HTMLButtonElement[]>([]);
function updateBlock() {
    const idx = active.value;
    const btn = btns.value[idx];
    const b = block.value;
    if (btn && b) {
        const rect = btn.getBoundingClientRect();
        const blockRect = b.getBoundingClientRect();
        if (blockRect) {
            b.style.left = `${rect.left}px`;
            b.style.width = `${rect.width}px`;
        }
    }
    const path = innerData[idx].path;
    if (!path) {
        return;
    }
    router.push(path);
}
watch(active, () => {
    updateBlock();
});
onMounted(() => {
    updateBlock();
});
defineExpose({
    active,
});
</script>
<style>
.tabs-outer-container {
    overflow: hidden;
    margin-bottom: 0px;
    height: 100%;
    position: relative;
    display: inline-block;
    flex: 1 1 auto;
    white-space: nowrap;
    overflow-x: hidden;
    width: 100%;
}
.tabs-container {
    height: 100%;
    display: flex;
}
.tabs-block {
    top: 0px;
    bottom: 0px;
    height: auto;
    border-radius: 4px;
    position: absolute;
    bottom: 0px;
    background-color: var(--main-color);
    transition: 300ms cubic-bezier(0.4, 0, 0.2, 1);
}
.tabs-button {
    display: inline-flex;
    -webkit-box-align: center;
    align-items: center;
    -webkit-box-pack: center;
    justify-content: center;
    box-sizing: border-box;
    -webkit-tap-highlight-color: transparent;
    background-color: transparent;
    outline: 0px;
    border: 0px;
    margin: 0px;
    border-radius: 0px;
    cursor: pointer;
    user-select: none;
    vertical-align: middle;
    appearance: none;
    text-decoration: none;
    font-family: inherit;
    font-weight: 500;
    line-height: calc(1.25 * var(--scale));
    text-transform: uppercase;
    max-width: calc(360px * var(--scale));
    position: relative;
    flex-shrink: 0;
    overflow: hidden;
    white-space: normal;
    text-align: center;
    flex-direction: column;
    color: var(--color);
    z-index: 1;
    padding: 4px 12px;
    min-height: 0px;
    min-width: 0px;
    font-size: calc(12px * var(--floor-scale));
}
.tabs-button:hover {
    color: var(--main-color);
}
.tabs-button.selected {
    color: var(--dark-color);
}
</style>
