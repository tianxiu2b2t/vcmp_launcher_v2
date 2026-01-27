<template>
    <InputEdit v-model:model-value="version"></InputEdit>
    {{ result }}
    <div v-for="item in progressbars">
        {{ JSON.stringify(item) }}
    </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import InputEdit from '../components/InputEdit.vue';
import { debounce } from 'vue-debounce';
import { downloadResource, getRandomObjectId } from '../bridge';
import { listen } from '@tauri-apps/api/event';
const version = ref('');
const result = ref('');
const progressbars = ref<object[]>([]);
watch(
    version,
    debounce(async () => {
        let id = await getRandomObjectId();
        listen(id, (event) => {
            progressbars.value.push(event.payload as object);
            console.log(event.payload);
        });
        result.value = await downloadResource(version.value, id);
    }, 1000),
);
</script>
