<script lang="ts" setup>
import { ElCard, ElAvatar } from 'element-plus';
import { ref } from 'vue';
import { Game, Participant, ParticipantIdentity } from '../models/LOL/Matches';
import { getChampionIconUrl, lget } from '../utils/lcu';
import { currentSummoner } from '../utils/global';

let props = defineProps<{
    gameId: number
}>();

interface FilterParticipant {
    championIcon: string,
    participant: Participant
    participantIdentity: ParticipantIdentity
}

let game = ref<Game>();
let participants = ref<FilterParticipant[]>([]);
let team1kill = ref(0);
let team2kill = ref(0);

async function init() {
    game.value = await lget<Game>(`/lol-match-history/v1/games/${props.gameId}`);
    for (let index = 0; index < game.value.participantIdentities.length; index++) {
        const participant = game.value.participants[index];
        const identity = game.value.participantIdentities[index];
        participants.value.push({
            championIcon: await getChampionIconUrl(participant.championId.toString()),
            participant,
            participantIdentity: identity
        })
        if (participant.teamId == 100) team1kill.value += participant.stats.kills;
        if (participant.teamId == 200) team2kill.value += participant.stats.kills;
    }
}
init();
</script>

<template>
    <el-card style="margin-top: 10px">
        <el-row justify="center" align="middle">
            <el-tag effect="dark" size="large">{{ team1kill }}</el-tag>
            <span style="margin: 0px 5px 0px 5px">VS</span>
            <el-tag effect="dark" size="large" type="danger">{{ team2kill }}</el-tag>
        </el-row>
    </el-card>
    <el-card style="margin-top: 10px; margin-bottom: 10px; border-width: 3px !important;"
        :body-style="{ padding: '5px 10px' }" v-for="p in participants" :class="`team-${p.participant.teamId}`">
        <el-row align="middle">
            <el-avatar :src="p.championIcon"> </el-avatar>
            <h3 style="margin: 5px 10px">
                {{ p.participantIdentity.player.summonerName }}</h3>
                <el-tag type="success" effect="dark" v-if="(currentSummoner?.displayName == p.participantIdentity.player.summonerName)">
                    You
                </el-tag>
        </el-row>
        <el-row style="margin-top: 5px" align="middle">
            <el-tag class="nearLeft">{{ p.participant.stats.kills }} kills / {{ p.participant.stats.deaths }} deaths /
            {{ p.participant.stats.assists }} assists</el-tag>
        </el-row>
    </el-card>
</template>

<style>
.team-100 {
    border-color: blue !important;
    background-color: rgb(156, 156, 255) !important;
}

.team-200 {
    border-color: red !important;
    background-color: rgb(255, 156, 156) !important;
}
</style>