<template>
    <!-- <InputEdit v-model:model-value="version"></InputEdit>
    {{ result }}
    <div v-for="item in progressbars">
        {{ JSON.stringify(item) }}
    </div> -->
    <InputEdit v-model:model-value="ip" placeholder="Server IP"></InputEdit>
    <InputEdit v-model:model-value="port" placeholder="Server Port"></InputEdit>
    <InputEdit v-model:model-value="version" placeholder="Version"></InputEdit>
    <InputEdit v-model:model-value="password" placeholder="Password (optional)"></InputEdit>
    <button @click="launchGame">Launch Game</button>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import InputEdit from '../components/InputEdit.vue';
import { debounce } from 'vue-debounce';
import { downloadResource, getRandomObjectId } from '../bridge';
import { listen } from '@tauri-apps/api/event';
import { launch } from '../bridge';
// const version = ref('');
// const result = ref('');
// const progressbars = ref<object[]>([]);
// watch(
//     version,
//     debounce(async () => {
//         let id = await getRandomObjectId();
//         listen(id, (event) => {
//             progressbars.value.push(event.payload as object);
//             console.log(event.payload);
//         });
//         result.value = await downloadResource(version.value, id);
//     }, 1000),
// );
const ip = ref('');
const port = ref('');
const version = ref('');
const password = ref('');
async function launchGame() {
    let pid = await launch({
        ip: ip.value,
        port: +port.value,
    }, version.value, password.value);
    console.log('Launched game with PID:', pid);
}
</script>
