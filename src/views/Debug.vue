<template>
    <InputEdit v-model:model-value="version"></InputEdit>
    {{ result }}
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import InputEdit from '../components/InputEdit.vue';
import { debounce } from 'vue-debounce';
import { downloadResource } from '../bridge';
import { listen } from '@tauri-apps/api/event';
const version = ref('');
const result = ref('');
watch(
    version,
    debounce(async () => {
        listen(`progressbar_${version.value}`, (event) => {
            console.log(event.payload);
        });
        result.value = await downloadResource(version.value);
    }, 1000),
);
</script>
