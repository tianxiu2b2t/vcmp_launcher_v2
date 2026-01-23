<template>
    <div
        :class="['inputbox-wrapper', wrapperClass, { active: effect }]"
        @click="focusInput"
    >
        <label
            :class="[
                'inputbox-label',
                labelClass,
                effect
                    ? 'inputbox-label-active'
                    : 'inputbox-label-inactive'
            ]"
        >
            {{ placeholder }}
        </label>

        <div
            :class="[
                'inputbox-input-container',
                inputWrapperClass,
            ]"
        >
            <component
                :is="mode === 'textarea' ? 'textarea' : 'input'"
                ref="inputRef"
                :type="mode === 'input' ? type : undefined"
                :name="input_name"
                :value="modelValue"
                :rows="mode === 'textarea' ? 1 : undefined"
                @focus="isFocus = true"
                @blur="isFocus = false"
                @input="handleInput"
                :class="[
                    'inputbox-input-element',
                    mode === 'input' ? 'inputbox-input-type' : 'inputbox-textarea-type',
                    inputClass,
                ]"
            />

            <fieldset class="inputbox-fieldset-element">
                <legend
                    :class="[
                        'inputbox-legend-element',
                        effect ? 'inputbox-legend-active' : '',
                    ]"
                >
                    <span class="inputbox-legend-text">{{ placeholder }}</span>
                </legend>
            </fieldset>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, nextTick } from 'vue';

const props = defineProps({
    modelValue: {
        type: String,
        default: '',
    },
    input_name: {
        type: String,
        default: '',
    },
    type: {
        type: String,
        default: 'text',
    },
    placeholder: {
        type: String,
        default: 'Enter something',
    },
    wrapperClass: {
        type: String,
        default: '',
    },
    inputClass: {
        type: String,
        default: '',
    },
    labelClass: {
        type: String,
        default: '',
    },
    inputWrapperClass: {
        type: String,
        default: '',
    },
    mode: {
        type: String as () => 'input' | 'textarea',
        default: 'input',
    },
});

const emit = defineEmits<{
    (e: 'update:modelValue', value: string): void;
}>();

const isFocus = ref(false);
const effect = ref(!!props.modelValue);
const inputRef = ref<HTMLInputElement | HTMLTextAreaElement | null>(null);

watch(isFocus, applyEffect);
watch(
    () => props.modelValue,
    () => {
        applyEffect();
        nextTick(() => {
            if (props.mode === 'textarea') adjustHeight();
        });
    },
);

function applyEffect() {
    effect.value = isFocus.value || !!props.modelValue;
}

function handleInput(event: Event) {
    const value = (event.target as HTMLInputElement | HTMLTextAreaElement)
        .value;
    emit('update:modelValue', value);
    if (props.mode === 'textarea') adjustHeight();
}

function adjustHeight() {
    const el = inputRef.value as HTMLTextAreaElement;
    if (!el || props.mode !== 'textarea') return;
    el.style.height = 'auto';
    el.style.height = `${el.scrollHeight}px`;
}

function focusInput() {
    const el = inputRef.value as HTMLInputElement | HTMLTextAreaElement;
    if (!el) return;
    el.focus();
}

onMounted(() => {
    if (props.mode === 'textarea') nextTick(adjustHeight);
});
</script>

<style scoped>
/* 完全按照原始TailwindCSS的类和顺序实现 */

/* 主容器 - 对应 :class="['relative w-full inputbox', wrapperClass, { active: effect }]" */
.inputbox-wrapper {
    position: relative;
    width: 100%;
}

/* 标签基础样式 - 完全对应原始Tailwind类 */
.inputbox-label {
    position: absolute;
    left: 0;
    top: 0;
    z-index: 10;
    font-size: 0.875rem; /* text-sm */
    line-height: 1.25rem;
    transition-property: all;
    transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    transition-duration: 200ms;
    transform-origin: left;
    color: rgba(0, 0, 0, 0.6); /* text-black/60 */
}

/* 深色模式标签颜色 - 对应 dark:text-white/70 */
.dark .inputbox-label {
    color: rgba(255, 255, 255, 0.7);
}

/* 标签激活状态 - 对应 :class="effect ? 'translate-x-[14px] -translate-y-[9px] scale-[0.75]'" */
.inputbox-label-active {
    transform: translateX(14px) translateY(-9px) scale(0.75);
}

/* 标签非激活状态 - 对应 :class="effect ? ... : 'translate-x-[14px] translate-y-[10px] scale-[1]'" */
.inputbox-label-inactive {
    transform: translateX(14px) translateY(10px) scale(1);
}

/* 输入框容器 - 对应 :class="['relative flex items-center w-full rounded', inputWrapperClass]" */
.inputbox-input-container {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
    border-radius: 0.25rem; /* rounded */
}

/* 输入框元素 - 对应 :class="['w-full px-[14px] py-[8.25px] bg-transparent outline-none', 'text-black dark:text-white', ...]" */
.inputbox-input-element {
    width: 100%;
    padding-left: 14px;
    padding-right: 14px;
    padding-top: 8.25px;
    padding-bottom: 8.25px;
    background-color: transparent;
    outline: 2px solid transparent;
    outline-offset: 2px;
    color: rgb(0, 0, 0); /* text-black */
    font-family: inherit;
    font-size: inherit;
    border: none;
}

/* 深色模式文本颜色 */
.dark .inputbox-input-element {
    color: rgb(255, 255, 255); /* dark:text-white */
}

/* input类型 - 对应 :class="mode === 'input' ? 'h-[40px]'" */
.inputbox-input-type {
    height: 40px;
}

/* textarea类型 - 对应 :class="mode === 'textarea' ? 'min-h-[40px] resize-none overflow-hidden'" */
.inputbox-textarea-type {
    min-height: 40px;
    resize: none;
    overflow: hidden;
}

/* fieldset - 对应 class="absolute inset-x-0 -top-[5px] px-2 pointer-events-none border border-none rounded" */
.inputbox-fieldset-element {
    position: absolute;
    left: 0;
    right: 0;
    top: -5px;
    padding-left: 0.5rem; /* px-2 = 8px */
    padding-right: 0.5rem;
    pointer-events: none;
    border-width: 1px;
    border-style: solid;
    border-color: transparent; /* border border-none 的实现 */
    border-radius: 0.25rem; /* rounded */
    margin: 0;
    bottom: 0;
}

/* legend - 对应 class="text-xs invisible max-w-[0.01px] overflow-hidden whitespace-nowrap transition-all duration-75" */
.inputbox-legend-element {
    font-size: 0.75rem; /* text-xs */
    line-height: 1rem;
    visibility: hidden; /* invisible */
    max-width: 0.01px;
    overflow: hidden;
    white-space: nowrap;
    transition-property: all;
    transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    transition-duration: 75ms;
    padding: 0;
}

/* legend激活状态 - 对应 :class="effect ? 'max-w-full' : ''" */
.inputbox-legend-active {
    max-width: 100%;
}

/* legend内的span - 对应 class="px-[5px] opacity-0 visible bg-white/10 dark:bg-black/10" */
.inputbox-legend-text {
    padding-left: 5px;
    padding-right: 5px;
    opacity: 0;
    visibility: visible;
    background-color: rgba(255, 255, 255, 0.1); /* bg-white/10 */
}

/* 深色模式legend背景 */
.dark .inputbox-legend-text {
    background-color: rgba(0, 0, 0, 0.1); /* dark:bg-black/10 */
}
</style>