<script lang="ts" setup>
import { Switch } from '@element-plus/icons-vue';
import { ElMessage } from 'element-plus';
import { SpellItem } from '../models/Backend/SelectChampion';
import { setCurrentSpell } from '../utils/lcu';

let props = defineProps<{
    spells: SpellItem[]
}>();

async function setSpells(reverse: boolean, spell: SpellItem) {
    let n: SpellItem = JSON.parse(JSON.stringify(spell))

    if (reverse) {
        n.ids.reverse()
    }
    await setCurrentSpell(n);
    ElMessage.success({
        message: reverse ? "Apply spell reversed success." : "Apply spell success.",
        grouping: true
    })
}
</script>

<template>
    <el-row v-for="spell in props.spells.slice(0, props.spells.length > 2 ? 2 : undefined)" align="middle">
        <el-card style="margin-top: 10px; margin-bottom: 10px;" class="maxWidth">
            <el-row align="middle">
                <el-avatar style="margin-left: 5px" shape="square" :size="32" v-for="p in spell.ids" :src="`/spells/${p}.png`">
                </el-avatar>
                <el-tag round class="nearLeft">
                    Win: {{ (spell.win * 100 / spell.play).toFixed(2) }}
                </el-tag>
                <el-tag round type="success" class="nearLeft">
                    Play: {{ spell.play }}
                </el-tag>
                <el-tooltip content="Click to apply the spell">
                    <el-button type="primary" :icon="Switch" circle class="nearLeft" @click="setSpells(false, spell)" />
                </el-tooltip>
                <el-tooltip content="Click to apply spell by revered">
                    <el-button type="warning" :icon="Switch" circle class="nearLeft" @click="setSpells(true, spell)" />
                </el-tooltip>
            </el-row>
        </el-card>
    </el-row>
</template>