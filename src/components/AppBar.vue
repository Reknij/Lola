<script setup lang="ts">
import { Close, Minus, Menu, ArrowLeftBold, BottomRight } from '@element-plus/icons-vue';
import { exit } from '@tauri-apps/api/process';
import { appWindow, LogicalSize } from '@tauri-apps/api/window';
import { inject, Ref, ref } from 'vue';
import { useRouter } from 'vue-router';
import appIcon from '../../appIcon.svg'
import { getAppConfig, saveAppConfig, setAppConfig } from './utils/appConfig';

let router = useRouter();
let miniMode = inject<Ref<boolean>>('miniMode');

async function closeApp() {
    await exit(1);
}

async function minimizeApp() {
    await appWindow.minimize();
}

async function minimizeToTray() {
    await appWindow.hide();
}

async function iconClick() {
    let config = await getAppConfig();
    if (miniMode) miniMode.value = config.mini_mode = !config.mini_mode;
    await setAppConfig(config);
    await saveAppConfig();
    console.log(config)
}

async function toMenu() {
    if (router.currentRoute.value.fullPath == '/') {
        router.push("/menu");
    } else {
        router.push("/");
    }
}
</script>

<template>
    <el-row align="middle" justify="space-between" data-tauri-drag-region>
        <el-row align="middle">
            <el-tooltip content="Click to enable mini mode." placement="right">
                <el-avatar id="appIconElement" :src="appIcon" @click="iconClick"></el-avatar>
            </el-tooltip>
            <h1 class="nearLeft">Lola</h1>
        </el-row>
        <div>
            <el-button :icon="router.currentRoute.value.path == '/menu' ? ArrowLeftBold : Menu" circle
                @click="toMenu" />
            <el-button :icon="BottomRight" circle @click="minimizeToTray" />
            <el-button :icon="Minus" circle @click="minimizeApp" />
            <el-button :icon="Close" type="danger" circle @click="closeApp" />
        </div>
    </el-row>
</template>

<style>
#appIconElement:hover {
    cursor: pointer;
}
</style>