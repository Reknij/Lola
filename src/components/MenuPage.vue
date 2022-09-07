<script lang="ts" setup>
import { Delete } from '@element-plus/icons-vue';
import { invoke } from '@tauri-apps/api';
import { appWindow } from '@tauri-apps/api/window';
import { ElMessage } from 'element-plus';
import { ref } from 'vue';
import { AppConfig } from './models/Backend/AppConfig';
import { getAppConfig, saveAppConfig, setAppConfig } from './utils/appConfig';

let activeName = ref("description");
let scrollbarHeight = ref(680);

let config = ref<AppConfig>();

async function init() {
    config.value = await getAppConfig();
}
init();
let options = [
    {
        label: "Online",
        value: "Online"
    },
    {
        label: "Auto",
        value: "Auto"
    }
]

async function save(val: string) {
    if (config.value) {
        await setAppConfig(config.value);
        await saveAppConfig();

        ElMessage.success({
            message: 'Save config success.',
            grouping: true
        })
    }
}

async function clearCache() {
    await invoke("clear_cache");
    ElMessage.success({
        message: 'Clear cache success.',
        grouping: true
    })
}

appWindow.onResized(s => {
    scrollbarHeight.value = 680 + (s.payload.height - 800);
})
</script>

<template>
    <el-scrollbar :height="scrollbarHeight">
        <el-collapse v-model="activeName" accordion>
            <el-collapse-item name="description">
                <template #title>
                    <h2>Description</h2>
                </template>
                <p class="item">This application is made by Jinker. Only for fun.</p>
            </el-collapse-item>

            <el-collapse-item name="championInfo" v-if="config">
                <template #title>
                    <h2>Champion information</h2>
                </template>
                <el-row align="middle">
                    <span class="item" style="font-weight:bold;">Fetch mode: </span>
                    <el-select class="nearLeft" v-model="config.fetch_mode" placeholder="Select" size="large">
                        <el-option v-for="item in options" :key="item.value" :label="item.label" :value="item.value" />
                    </el-select>
                    <el-tooltip v-if="config.fetch_mode == 'Auto'" content="Clear all saved cache." placement="top">
                        <el-button class="nearLeft" circle :icon="Delete" @click="clearCache"></el-button>
                    </el-tooltip>
                </el-row>
                <el-row align="middle" style="margin-top: 10px" v-if="config.fetch_mode == 'Auto'">
                    <span class="item" style="font-weight:bold;">Cache expired: </span>
                    <el-input-number class="nearLeft" :min="3" :max="90" v-model="config.expired_days">
                    </el-input-number>
                    <span class="nearLeft" style="font-weight:bold;">days</span>
                </el-row>
                <el-row align="middle" style="margin-top: 10px">
                    <el-button class="nearLeft" @click="save">Save</el-button>
                </el-row>
            </el-collapse-item>

            <el-collapse-item name="information">
                <template #title>
                    <h2>App information</h2>
                </template>
                <h3>Version: v0.1</h3>
                <h3>Author: Jinker</h3>
                <h3>Data path: <el-tag type="info">{{ config?.data_path }}</el-tag>
                </h3>
                <h3>Cache path: <el-tag type="info">{{ config?.cache_path }}</el-tag>
                </h3>
                <h3>Config path: <el-tag type="info">{{ config?.config_path }}</el-tag>
                </h3>
            </el-collapse-item>
        </el-collapse>
    </el-scrollbar>
</template>

<style>
.item {
    font-size: medium;
}
</style>