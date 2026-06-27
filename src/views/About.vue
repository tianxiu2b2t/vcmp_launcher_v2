<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import SvgIcon from '../components/SvgIcon.vue';

const textRef = ref<HTMLSpanElement | null>(null);
const iconWidth = ref('24px'); // 默认宽度
let resizeObserver: ResizeObserver | null = null;

// 更新SVG宽度的函数
const updateIconWidth = () => {
    if (textRef.value && textRef.value.offsetWidth > 0) {
        // 获取文字的实际宽度
        iconWidth.value = `${textRef.value.offsetWidth}px`;
    }
};

onMounted(() => {
    // 初始更新
    updateIconWidth();

    // 创建ResizeObserver监听文字宽度变化
    if (textRef.value) {
        resizeObserver = new ResizeObserver((entries) => {
            for (const entry of entries) {
                if (entry.contentRect) {
                    updateIconWidth();
                }
            }
        });

        resizeObserver.observe(textRef.value);
    }

    // 也可以监听窗口变化
    window.addEventListener('resize', updateIconWidth);
});

onUnmounted(() => {
    if (resizeObserver && textRef.value) {
        resizeObserver.unobserve(textRef.value);
        resizeObserver.disconnect();
    }
    window.removeEventListener('resize', updateIconWidth);
});
</script>

<template>
    <div class="heading">
        <span class="clip">Vice City Multiplayer Launcher V2</span>
        <div class="built-by-container">
            <span ref="textRef" class="text-content">
                {{ $t('about.built_by') }}
            </span>
            <!-- SVG图标作为背景装饰 -->
            <div class="svg-background">
                <SvgIcon name="Hero" :width="iconWidth" height="24px" />
            </div>
        </div>
        <span>{{ $t('about.features') }}</span>
    </div>
</template>

<style scoped>
:root {
    --c: linear-gradient(
        45deg,
        var(--main-color) 15%,
        var(--dark-main-color) 50%
    );
}
.heading {
    line-height: 64px;
    font-size: 40px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    width: auto;
}

.built-by-container {
    position: relative;
    display: inline-block;
    height: 64px; /* 确保容器有高度，以便SVG可以定位 */
}

.text-content {
    position: relative;
    z-index: 2;
    white-space: nowrap;
    line-height: 64px; /* 与heading保持一致 */
}

.svg-background {
    position: absolute;
    top: calc(100% - 36px); /* 定位到文字下方，稍微有点间距 */
    left: 0;
    width: 100%;
    pointer-events: none; /* 不干扰鼠标事件 */
    z-index: 1; /* 在文字下层 */
    fill: var(--main-color);
}

.clip {
    background: linear-gradient(
        45deg,
        var(--main-color) 15%,
        var(--dark-main-color) 50%
    );
    background-clip: text;
    -webkit-text-fill-color: transparent;
}
</style>
