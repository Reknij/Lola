<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { appWindow, PhysicalSize } from '@tauri-apps/api/window';
import { provide, ref } from 'vue';
import appBar from './components/AppBar.vue';
import appIcon from '../appIcon.svg'
import MiniUI from './components/MiniUI.vue';
import { getAppConfig } from './components/utils/appConfig';

let miniMode = ref(false);
let miniState = ref(false);
let imgSource = ref(appIcon)
appWindow.setAlwaysOnTop(true);

function setMiniState(val: boolean) {
  if (!miniMode.value) return;
  miniState.value = val;
  if (val) appWindow.setSize(new PhysicalSize(64, 64));
  else appWindow.setSize(new PhysicalSize(500, 800));
}
provide("miniMode", miniMode);
provide("setMiniState", setMiniState);
provide("setMiniImg", (source?: string) => {
  if (!source) source = appIcon;
  imgSource.value = source;
})

let body = document.getElementsByTagName("BODY")[0];
body.addEventListener('mouseleave', e => {
  setMiniState(true);
})

async function init() {
  miniMode.value = (await getAppConfig()).mini_mode;
}
init();

</script>

<template>
  <el-container v-show="!miniState">
    <el-header>
      <app-bar></app-bar>
    </el-header>
    <el-main>
      <router-view></router-view>
    </el-main>
  </el-container>
  <MiniUI :img="imgSource" v-show="miniState"></MiniUI>
</template>

<style>
.nearLeft {
  margin-left: 10px;
}
.maxWidth {
  width: 100vw;
}
</style>
