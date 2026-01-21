<template>
    <div class="progressbar">
        <div 
            class="inner" 
            :style="{ width: `${progressbar}%` }"
            :class="{ 'complete': progressbar >= 100 }"
        ></div>
    </div>
</template>

<script lang="ts" setup>
import { computed, watch, ref } from 'vue';
import { progressbar as outer_progressbar } from '../constants';

const progressbar = computed(() => {
    const value = outer_progressbar.value;
    if (value < 0) return 0;
    if (value > 100) return 100;
    return value;
});

const isComplete = ref(false);

watch(progressbar, (newVal) => {
    if (newVal >= 100) {
        isComplete.value = true;
        // 可选：进度完成后自动隐藏
        setTimeout(() => {
            // 这里可以触发一个事件或重置进度
        }, 500);
    } else {
        isComplete.value = false;
    }
});
</script>

<style scoped>
.progressbar {
    position: relative;
    width: 100%;
    height: 3px;
    background-color: transparent;
    overflow: hidden;
}

.inner {
    height: 100%;
    width: 0;
    background: linear-gradient(90deg, 
        var(--main-color) 0%, 
        color-mix(in srgb, var(--main-color) 80%, white) 100%
    );
    transition: width 300ms cubic-bezier(0.4, 0, 0.2, 1);
    border-radius: 2px;
    position: relative;
}

.inner.complete {
    opacity: 0;
    transition: width 300ms cubic-bezier(0.4, 0, 0.2, 1), 
                opacity 500ms ease 300ms;
}

/* 添加光泽效果 */
.inner::after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(
        90deg,
        transparent,
        rgba(255, 255, 255, 0.3),
        transparent
    );
    transform: translateX(-100%);
    animation: shine 2s infinite;
}

@keyframes shine {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
}
</style>