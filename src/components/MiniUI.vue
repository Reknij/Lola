<script lang="ts" setup>
import { inject, Ref } from 'vue';

let props = defineProps<{
    img: string
}>();

let setMiniState: any = inject("setMiniState")
let miniMode = inject<Ref<boolean>>('miniMode');

if (miniMode?.value) {
    setMiniState(true);
}

function enter() {
    setMiniState(false);
}

</script>

<template>
    <el-avatar id="miniImg" :size="48" :src="props.img" @mouseenter="enter"></el-avatar>
</template>

<style>

body {
    overflow: hidden;
}
</style>