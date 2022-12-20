<script lang="ts" setup>import { appWindow } from '@tauri-apps/api/window';
import { onUnmounted, ref } from 'vue';
import { Matches } from './models/LOL/Matches';
import { lget } from './utils/lcu';
import History from './MatchHistory/History.vue';
import { listen, UnlistenFn, TauriEvent } from '@tauri-apps/api/event';

let matches = ref<Matches>();
let beginIndex = 0;
let endIndex = 20;

let unlisten = ref<UnlistenFn>(() => { });

let scrollbarHeight = ref(620);

appWindow.onResized((s) => {
    scrollbarHeight.value = 620 + (s.payload.height - 800);
});

async function loadMatches() {
    matches.value = await lget<Matches>(`/lol-match-history/v1/products/lol/current-summoner/matches?begIndex=${beginIndex}&endIndex=${endIndex}`);
}

async function init() {
    loadMatches();
    unlisten.value = await listen(TauriEvent.WINDOW_FOCUS, async e => {
        await loadMatches();
    })
}

init();

onUnmounted(() => {
    unlisten.value();
})
</script>

<template>
    <el-scrollbar :height="scrollbarHeight">
        <History :matches="matches"></History>
    </el-scrollbar>
</template>