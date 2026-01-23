<script setup lang="ts">
import Progressbar from './components/Progressbar.vue';

</script>

<template>
	<Progressbar/>
	<RouterView></RouterView>
</template>

<style>
:root.dark {
	--bg-color: #1f1f1f;
	--main-color: #f4d1b4;
	color: #f5f5f5;
}
:root {
	--bg-color: #f5f5f5;
	--main-color: #0fc6c2;
}
body {
	background-color: var(--bg-color);
}
</style>