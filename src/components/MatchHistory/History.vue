<script lang="ts" setup>
import { Back } from '@element-plus/icons-vue';
import { Ref, ref, watch } from 'vue';
import { Game, Matches } from '../models/LOL/Matches';
import { getChampionIconUrl, getGameName } from '../utils/lcu';
import Detail from './Detail.vue';

let props = defineProps<{
    matches: Matches | undefined
}>();

let gameId = ref<number>();

async function clickDetail(game: Game) {
    gameId.value = game.gameId
}

interface FilterGame {
    championIcon: string,
    game: Game
}

let filterGames = ref<FilterGame[]>([]);
watch(() => props.matches, loadGames)
async function loadGames() {
    filterGames.value.length = 0;
    if (!props.matches) return;
    for (let index = 0; index < props.matches.games.games.length; index++) {
        const g = props.matches?.games.games[index];
        filterGames.value.push({
            championIcon: await getChampionIconUrl(g.participants[0].championId.toString()),
            game: g
        })
    }
    filterGames.value.sort((g1, g2) => {
        let date1 = Date.parse(g1.game.gameCreationDate);
        let date2 = Date.parse(g2.game.gameCreationDate);
        return date2 - date1;
    })
}
loadGames();

function formatDate(dateText: string): string {
    let date = new Date(dateText);
    return `${date.getFullYear()}/${date.getMonth() + 1}/${date.getDate()} - ${date.getHours()}:${date.getMinutes()}:${date.getSeconds()}`
}
</script>

<template>
    <el-button v-if="gameId" round :icon="Back" @click="gameId = undefined"></el-button>
    <el-timeline v-if="!gameId">
        <el-timeline-item v-for="(fg, i) in filterGames" :key="i"
            :color="`${fg.game.participants[0].stats.win ? 'blue' : 'red'}`" center
            :timestamp="formatDate(fg.game.gameCreationDate)" style="cursor: pointer;" @click="clickDetail(fg.game)">
            <el-card :body-style="{ padding: '5px 10px' }"
                style="border-width: 3px !important; margin-top: 10px; margin-bottom: 10px;"
                :class="`team-${fg.game.participants[0].stats.win ? 'Win' : 'Fail'}`">
                <el-row align="middle">
                    <el-avatar :src="fg.championIcon"></el-avatar>
                    <el-tag class="nearLeft">{{ fg.game.participants[0].stats.kills }} kills / {{
                            fg.game.participants[0].stats.deaths
                    }}
                        deaths /
                        {{ fg.game.participants[0].stats.assists }} assists</el-tag>
                </el-row>
                <el-row justify="space-between" align="middle">
                    <el-tag effect="dark" type="warning">{{ `${getGameName(fg.game.queueId)}`
                    }}</el-tag>
                    <span :class="`winstat winstat-${fg.game.participants[0].stats.win ? 'v' : 'd'}`">{{ fg.game.participants[0].stats.win ? 'Victory' : 'Defeat' }}</span>
                </el-row>
            </el-card>
        </el-timeline-item>
    </el-timeline>

    <Detail :game-id="gameId" v-else></Detail>
</template>

<style>
.team-Win {
    background-color: rgba(0, 0, 255, 0.5) !important;
    border-color: blue !important;
}

.team-Fail {
    background-color: rgba(255, 0, 0, 0.5) !important;
    border-color: red !important;
}

.winstat {
    font-weight: bold;
    font-size: xx-large;
}

.winstat-v {
    color: blue;
}

.winstat-d {
    color: red;
}
</style>