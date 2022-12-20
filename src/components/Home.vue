<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window';
import { ref } from '@vue/reactivity';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api';
import AmumuLoading from './AmumuLoading.vue';
import { onUnmounted, watch } from 'vue';
import ChampionBuild from './ChampionBuild.vue';
import MatchHistory from './MatchHistory.vue';

let activeTab = ref("dashboard");
let unlisten = ref<UnlistenFn>(()=>{});

async function clientSelecting() {
  await appWindow.setFocus();
  activeTab.value = "championBuild";
}

let initialized = ref(false);
let appStatus = ref();
async function init() {
  appStatus.value = await invoke("get_app_status");
  unlisten.value = await listen<any>("app_status_changed", async e => {
  appStatus.value = e.payload;
})
}
init();
watch(appStatus, async s => {
  initialized.value = s.lcu_loaded;
  if (initialized.value) {
    await invoke("initialize");
  }
})

onUnmounted(()=>{
  unlisten.value();
})

</script>

<template>
  <el-tabs v-if="initialized" v-model="activeTab">
    <el-tab-pane label="Dashboard" name="dashboard">
      <dashboard></dashboard>
    </el-tab-pane>
    <el-tab-pane label="Champion build" name="championBuild">
      <champion-build :clientSelecting="clientSelecting"></champion-build>
    </el-tab-pane>
    <el-tab-pane label="Match History" name="matchHistory">
      <MatchHistory></MatchHistory>
    </el-tab-pane>
  </el-tabs>
  <amumu-loading v-else>
    <h4>Please ensure League Of Legends is started.</h4>
  </amumu-loading>
</template>