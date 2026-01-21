<script setup lang="ts">
import { greet } from '../bridge';

</script>

<template>
    <h4>{{ await greet("cnm") }}</h4>
</template>