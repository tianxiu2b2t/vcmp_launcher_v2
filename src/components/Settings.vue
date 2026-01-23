<template>
    123
    456
    <InputEdit
    placeholder="masterlist"/>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { getConfig } from '../bridge';
import InputEdit from './InputEdit.vue';
import { Config } from '../types';
const _config = ref<Config>(await getConfig())
const masterUrl = ref(_config.value.internet.master_url)
const updateUrl = ref(_config.value.internet.update_url);
const password = ref(_config.value.internet.password);
const username = ref(_config.value.game.username);
const game_dir = ref(_config.value.game.game_dir);
</script>