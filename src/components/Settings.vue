<template>
    123
    456
    <InputEdit
    placeholder="互联网服务器获取网址"
    :model-value="masterUrl"
    @update:model-value="`masterUrl`"/>
    <InputEdit
    placeholder="更新客户端获取网址"
    :model-value="updateUrl"
    :update:model-value="updateUrl"/>
    <InputEdit
    placeholder="更新客户端密码"
    :model-value="password"
    :update:model-value="password"/>
    <InputEdit
    placeholder="游戏目录"
    :model-value="gameDir"
    :update:model-value="gameDir"/>
    <InputEdit
    placeholder="用户名"
    :model-value="username"
    :update:model-value="username"/>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import InputEdit from './InputEdit.vue';
import { config } from '../constants';
// const outputConfig = computed<Config>(() => {
//     return {
//         ...(_config.value || {}),
//         internet: {
//             master_url: masterUrl.value || '',
//             update_url: updateUrl.value || '',
//             password: password.value || undefined,
//         },
//         game: {
//             game_dir: gameDir.value,
//             username: username.value,
//         }
//     }
// }) 
const masterUrl = ref<string>(config.value.internet.master_url);
const updateUrl = ref<string>(config.value.internet.update_url);
const password = ref<string>(config.value.internet.password || '');
const gameDir = ref<string>(config.value.game.game_dir || '');
const username = ref<string>(config.value.game.username || '');
watch(masterUrl, (newVal) => {
    console.log(newVal)  
})
// console.log(_config)
// onBeforeMount(async () => {
//     _config.value = await getConfig()
//     console.log(_config.value)
//     masterUrl.value = _config.value.internet.master_url
//     updateUrl.value = _config.value.internet.update_url
//     password.value = _config.value.internet.password
//     gameDir.value = _config.value.game.game_dir
//     username.value = _config.value.game.username
// })
// onMounted(() => {
//     watch(outputConfig, async (newVal) => {
//         console.log(newVal)
//     // await setConfig(newVal)  
// })
// })
</script>