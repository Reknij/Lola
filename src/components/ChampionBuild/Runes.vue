<script lang="ts" setup>
import { Delete, Switch } from '@element-plus/icons-vue';
import { ElMessage } from 'element-plus';
import { RuneItem } from '../models/Backend/SelectChampion';
import { removeChampionCustomRune, setCurrentRune } from '../utils/lcu';

let props = defineProps<{
    runes: RuneItem[],
    isCustom: boolean,
    championName: string,
    selectedLane: string,
    gameMode: string,
}>();

async function setRune(rune: RuneItem) {
    await setCurrentRune(rune);
    ElMessage.success({
        message: "Apply rune success.",
        grouping: true
    })
}

async function removeRune(rune: RuneItem) {
    if (await removeChampionCustomRune(props.championName, props.gameMode, rune.id)) {
        for (let index = 0; index < props.runes.length; index++) {
            const r = props.runes[index];
            if (r.id == rune.id) {
                props.runes.splice(index, 1);
                break;
            }
        }
        ElMessage.success({
            message: "Remove rune success.",
            grouping: true
        })
    }
    else {
        ElMessage.warning({
            message: "Remove rune failed because not found rune id in custom!",
            grouping: true
        })
    }
}

function getBorderColorByRuneId(runeId: number): string {
    if (runeId == 8000) return '#fcf1c3';
    if (runeId == 8100) return '#d54343';
    if (runeId == 8200) return '#9ba6fc';
    if (runeId == 8300) return '#58cee0';
    if (runeId == 8400) return '#bdfb9e';
    else return 'black'
}
</script>

<template>
    <el-row v-for="rune in props.runes" align="middle" justify="center">
        <el-card style="margin-top: 10px; margin-bottom: 10px;n" class="maxWidth">
            <h2 v-if="props.isCustom">{{rune.name}}</h2>
            <el-row align="middle">
                <el-avatar :size="68" style="background-color: black;" :style="{'border': `2px solid ${getBorderColorByRuneId(rune.primary_page_id)}`}">
                    <el-avatar class="runeOrSpell" :size="48" :src="`/runes/${rune.primary_page_id}.png`"></el-avatar>
                </el-avatar>
                <el-avatar class="nearLeft runeOrSpell" :style="{'border': `2px solid ${getBorderColorByRuneId(rune.primary_page_id)}`}" :size="48" v-for="p in rune.primary_rune_ids" :src="`/runes/${p}.png`">
                </el-avatar>
            </el-row>
            <el-row align="middle" style="margin-top: 15px;">
                <el-avatar :size="68" style="background-color: black;" :style="{'border': `2px solid ${getBorderColorByRuneId(rune.secondary_page_id)}`}">
                    <el-avatar class="runeOrSpell" :size="48" :src="`/runes/${rune.secondary_page_id}.png`"></el-avatar>
                </el-avatar>
                <el-avatar class="nearLeft runeOrSpell" :style="{'border': `2px solid ${getBorderColorByRuneId(rune.secondary_page_id)}`}" :size="48" v-for="p in rune.secondary_rune_ids" :src="`/runes/${p}.png`">
                </el-avatar>
                <el-avatar class="nearLeft runeOrSpell" style="border: 2px solid gold;" :size="32" v-for="p in rune.stat_mod_ids" :src="`/runes/${p}.png`">
                </el-avatar>
            </el-row>
            <el-row align="middle" style="margin-top: 15px;">
                <div v-if="!props.isCustom">
                    <el-tag round class="nearLeft">
                        Win: {{ (rune.win * 100 / rune.play).toFixed(2) }}
                    </el-tag>
                    <el-tag round type="success" class="nearLeft">
                        Play: {{ rune.play }}
                    </el-tag>
                </div>

                <el-button type="primary" :icon="Switch" circle class="nearLeft" @click="setRune(rune)" />
                <el-button type="danger" v-if="props.isCustom" :icon="Delete" circle class="nearLeft"
                    @click="removeRune(rune)" />
            </el-row>
        </el-card>
    </el-row>
</template>