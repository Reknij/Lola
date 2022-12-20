<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { appWindow } from '@tauri-apps/api/window';
import { ref } from 'vue';
import { ChampionInfo } from './models/LOL/ChampionInfo';
import { Mastery } from './models/LOL/Materies';
import { SummonerInfo } from './models/LOL/SummonerInfo';
import { currentSummoner, summonerIconUrl } from './utils/global';
import { getChampionIconUrl, lget } from './utils/lcu';

interface ChampionMasteryTop {
    masteries: Mastery[],
    score: number,
    summonerId: number
}

let masteriesTop = ref<ChampionMasteryTop>();
let masteriesTopInfo = ref<any>([])
let scrollbarHeight = ref(480)

function parseGrade(highestGrade: string): string {
    highestGrade = highestGrade.toLowerCase();
    if (highestGrade[0] == 's') {
        return "warning";
    }
    else return "success";
}

async function init() {
    currentSummoner.value = await lget<SummonerInfo>("/lol-summoner/v1/current-summoner");
    summonerIconUrl.value = `https://cdn.communitydragon.org/latest/profile-icon/${currentSummoner.value.profileIconId}`

    let masteries: ChampionMasteryTop = await lget(`/lol-collections/v1/inventories/${currentSummoner.value.summonerId}/champion-mastery/top?limit=10`);
    if (masteries) {
        for (let index = 0; index < masteries.masteries.length; index++) {
            const element = masteries.masteries[index];
            let info: ChampionInfo = await invoke("get_champion_raw_info", {
                championId: element.championId.toString()
            });
            let id = info.id.toLowerCase();
            (info as any).icon = await getChampionIconUrl(element.championId.toString());
            masteriesTopInfo.value.push(info);
        }
        masteriesTop.value = masteries; // set here to refresh together masteriesTopInfo.
    }
}
init();
appWindow.onResized(s=>{
    scrollbarHeight.value = 480 + (s.payload.height - 800);
})
</script>

<template>
    <div>
        <el-row justify="center">
            <el-tooltip
                :content="`${currentSummoner?.xpSinceLastLevel} / ${currentSummoner?.xpUntilNextLevel} to level ${(currentSummoner?.summonerLevel ?? 0) + 1}.`"
                placement="bottom">
                <el-progress type="circle" :percentage="currentSummoner?.percentCompleteForNextLevel">
                    <el-avatar :size="100" :src="summonerIconUrl" />
                </el-progress>
            </el-tooltip>
            <div style="margin-left: 10px;">
                <h1>{{ currentSummoner?.displayName }}</h1>
                <el-tag effect="dark" round> Level {{ currentSummoner?.summonerLevel }}</el-tag>
            </div>
        </el-row>
        <el-row style="margin-top: 15px;" justify="start">
            <el-scrollbar :height="scrollbarHeight">
                <el-timeline>
                <el-timeline-item class="mastery" v-for="(mastery, index) in (masteriesTop?.masteries)" :key="index" hide-timestamp center>
                    <el-card>
                        <el-row justify="start" align="middle">
                            <el-badge :value="mastery.championLevel">
                                <el-avatar :src="masteriesTopInfo[index].icon">
                                </el-avatar>
                            </el-badge>
                            <span class="nearLeft" style="font-weight:bold;">{{ masteriesTopInfo[index].name }}</span>
                            <el-tag class="nearLeft" effect="dark" round :type="parseGrade(mastery.highestGrade)">{{ mastery.highestGrade }}</el-tag>
                            <el-tag class="nearLeft" round :type="index == 0? 'warning': ''">
                                <h2>{{mastery.championPoints}}</h2>
                            </el-tag>
                        </el-row>
                    </el-card>
                </el-timeline-item>
            </el-timeline>
            </el-scrollbar>
        </el-row>
    </div>
</template>

<style>
.mastery {
    width: calc(100vw - 100px);
}
</style>