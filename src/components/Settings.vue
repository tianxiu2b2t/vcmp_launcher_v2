<template>
    123
    456
    <InputEdit
    placeholder="互联网服务器获取网址"
    v-model:model-value="config.internet.master_url"/>
    <InputEdit
    placeholder="更新客户端获取网址"
    v-model:model-value="config.internet.update_url"/>
    <InputEdit
    placeholder="更新客户端密码"
    v-model:model-value="config.internet.password"/>
    <InputEdit
    placeholder="游戏目录"
    v-model:model-value="config.game.game_dir"/>
    <InputEdit
    placeholder="用户名"
    v-model:model-value="config.game.username"/>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import InputEdit from './InputEdit.vue';
import { config } from '../constants';
</script>