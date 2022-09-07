<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs

import { invoke } from "@tauri-apps/api";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { inject, onMounted, onUnmounted, ref } from "vue";
import { LcuEvents } from "./models/LOL/LcuEvents.js";
import { Build, RuneItem, SpellItem } from "./models/Backend/SelectChampion.js";
import {
  addChampionCustomRune,
  getChampionIconUrl,
  getChampionIdFromSession,
  getCurrentGameMode,
  getCurrentRune,
  lget,
  removeChampionCustomRunes,
  setCurrentRune,
  setCurrentSpell,
} from "./utils/lcu.js";
import { appWindow } from "@tauri-apps/api/window";
import { Switch, Refresh, Aim, Plus, Delete, Warning } from "@element-plus/icons-vue";
import AmumuLoading from "./AmumuLoading.vue";
import { ClientSelecting } from "../my";
import { ElMessage } from "element-plus";
import { ChampionInfo } from "./models/LOL/ChampionInfo";
import Runes from "./ChampionBuild/Runes.vue";
import Spells from "./ChampionBuild/Spells.vue";
import { LolRuneItem } from "./models/LOL/LolRuneItem";

const props = defineProps<{
  clientSelecting: ClientSelecting;
}>();

let currentRune = ref<LolRuneItem>();
let dialogVisible = ref(false);
let setMiniImg: any = inject("setMiniImg");
let wasteTime = ref(0);
let build = ref<Build>();
let championInfo = ref<ChampionInfo>();
let scrollbarHeight = ref(580);
let selectedLane = ref("");
let selectDisable = ref(false);
let loading = ref(true);
let onChampSelect = ref(false);
let lanes = [
  {
    label: "Top",
    value: "top",
  },
  {
    label: "Jungle",
    value: "jungle",
  },
  {
    label: "Mid",
    value: "mid",
  },
  {
    label: "Adc",
    value: "adc",
  },
  {
    label: "Support",
    value: "support",
  },
  {
    label: "Custom",
    value: "custom",
  },
];
let iconUrl = ref("/champions/empty.png");
let gameMode = ref("UNKNOWN");
let unlisten = ref<UnlistenFn>(() => { });

async function load_champ(d: any) {
  let championId = await getChampionIdFromSession(d);
  if (championId != 0 && championId.toString() != championInfo.value?.key) {
    clearInfo();

    championInfo.value = await invoke("get_champion_raw_info", {
      championId: championId.toString(),
    });
    if (!championInfo.value) return;
    iconUrl.value = championInfo.value.key
      ? await getChampionIconUrl(championInfo.value.key)
      : "/champions/empty.png";
    setMiniImg(iconUrl.value);
    loading.value = true;

    if (props.clientSelecting) {
      await props.clientSelecting();
      await appWindow.show();
      await appWindow.unminimize();
      await appWindow.setFocus();
    }

    if (gameMode.value != "CLASSIC") {
      await loadBuild("top"); //Aram and Urf mode no need lane, so give random lane to it.
    } else {
      let start = Date.now();
      await autoSelect();
      wasteTime.value = (Date.now() - start) / 1000;
    }
  }
}

async function init() {
  onChampSelect.value =
    (await lget<string>("/lol-gameflow/v1/gameflow-phase")) == "ChampSelect";
  gameMode.value = await getCurrentGameMode();
  let d = await lget<any>("/lol-champ-select/v1/session");
  await load_champ(d);

  unlisten.value = await listen<LcuEvents>("lcu_events", async (e) => {
    if (e.payload.uri == "/lol-gameflow/v1/gameflow-phase") {
      if (e.payload.data == "ChampSelect") {
        gameMode.value = await getCurrentGameMode();
        onChampSelect.value = true;
      } else onChampSelect.value = false;
      clearInfo();
    }

    if (e.payload.uri == "/lol-champ-select/v1/session") {
      await load_champ(e.payload.data);
    }
  });
}
init();

async function clearInfo() {
  loading.value = true;
  selectedLane.value = "";
  championInfo.value = undefined;
  iconUrl.value = "/champions/empty.png";
  setMiniImg();
}

async function addCurrentRune() {
  let current = await getCurrentRune();
  if (!current) {
    console.log("get current rune null.")
    return;
  }
  if (!championInfo.value) {
    ElMessage.warning({
      message: "Lol client must be selecting champion first!",
      grouping: true,
    });
    return;
  }
  dialogVisible.value = true;
  currentRune.value = current;
}

async function confirmAdd() {
  if (!championInfo.value) {
    ElMessage.warning({
      message: "Lol client must be selecting champion first!",
      grouping: true,
    });
    return;
  }
  if (!currentRune.value) {
    ElMessage.warning({
      message: "Current rune is null!",
      grouping: true,
    });
    return;
  }

  if (await addChampionCustomRune(championInfo.value.id, gameMode.value, currentRune.value)) {
    ElMessage.success({
      message: "Add current rune to custom success!",
      grouping: true,
    });
    await loadBuild(selectedLane.value)
  }
  else {
    ElMessage.warning({
      message: "Add failed because current id already contain in custom!",
      grouping: true,
    });
  }

  dialogVisible.value = false;
}

async function removeAllRune() {
  if (!championInfo.value) {
    ElMessage.warning({
      message: "Lol client must be selecting champion first!",
      grouping: true,
    });
    return;
  }
  build.value?.runes.splice(0, build.value.runes.length);
  if (await removeChampionCustomRunes(championInfo.value.id, gameMode.value)) {
    ElMessage.success({
      message: "Remove all custom rune for champion success!",
      grouping: true,
    });
  }
  else {
    ElMessage.warning({
      message: "Remove all custom rune for champion failed.",
      grouping: true,
    });
  }
}

async function loadBuild(val?: string) {
  if (val) {
    if (val == "") {
      ElMessage.warning({
        message: "Please select your lane first!",
        grouping: true,
      });
      return;
    }
    if (!championInfo.value) {
      ElMessage.warning({
        message: "Lol client must be selecting champion first!",
        grouping: true,
      });
      return;
    }
    loading.value = true;
    build.value = await loadBuildCore(
      championInfo.value.name,
      val,
      gameMode.value
    );
    loading.value = false;
  }
}

async function loadBuildCore(
  championName: string,
  lane: string,
  mode: string
): Promise<Build> {
  return (
    builds.get(lane) ??
    (await invoke<Build>("get_champion_build", {
      championName,
      lane,
      mode,
    }))
  );
}

let builds: Map<string, Build> = new Map();
async function autoSelect() {
  if (!championInfo.value) {
    ElMessage.warning({
      message: "Lol client must be selecting champion first!",
      grouping: true,
    });
    return;
  }
  selectDisable.value = true;
  loading.value = true;

  let bs = await invoke<Build[]>("get_champion_all_build", {
    championName: championInfo.value.id,
    mode: gameMode.value,
  });
  let last: Build | undefined = undefined;
  for (let index = 0; index < bs.length; index++) {
    const b = bs[index];
    if (b.runes.length > 0) {
      if (!last || b.runes[0].play > last.runes[0].play) {
        last = b;
      }
    }
    builds.set(b.lane, b);
  }

  if (last) {
    selectedLane.value = last.lane;
    build.value = last;
  }

  loading.value = false;
  selectDisable.value = false;
}

appWindow.onResized((s) => {
  scrollbarHeight.value = 580 + (s.payload.height - 800);
});

onUnmounted(() => {
  unlisten.value();
});
</script>

<template>
  <AmumuLoading v-if="!onChampSelect">
    <h4>
      This function must ensure Lol Client is on champion selecting state.
    </h4>
  </AmumuLoading>
  <div v-else>
    <el-row justify="center" align="middle">
      <el-avatar :src="iconUrl"> </el-avatar>
      <span class="nearLeft">build from lane {{ gameMode }}</span>
      <div v-if="gameMode == 'CLASSIC'">
        <el-select :disabled="selectDisable" style="width: 100px" v-model="selectedLane" @change="loadBuild"
          class="nearLeft">
          <el-option v-for="item in lanes" :key="item.value" :label="item.label" :value="item.value" />
        </el-select>
        <el-button :icon="Refresh" class="nearLeft" circle @click="loadBuild(selectedLane)"></el-button>
      </div>
      <div v-else>
        <el-tag class="nearLeft" type="info">All</el-tag>
        <el-button :icon="Refresh" class="nearLeft" circle @click="loadBuild('top')"></el-button>
      </div>
      <el-button :icon="Aim" class="nearLeft" circle @click="autoSelect"></el-button>
    </el-row>
    <el-row align="middle" justify="center" v-if="selectedLane == 'custom'">
      <el-dialog v-model="dialogVisible" v-if="currentRune">
        <template #header>
          <p>Enter your custom rune name:</p>
        </template>
        <el-input v-model="currentRune.name"></el-input>
        <template #footer>
          <span class="dialog-footer">
            <el-button @click="dialogVisible = false">Cancel</el-button>
            <el-button type="primary" @click="confirmAdd">Add</el-button>
          </span>
        </template>
      </el-dialog>
      <el-tooltip content="Add current rune to custom.">
        <el-button style="margin-bottom: 10px" :icon="Plus" class="nearLeft" circle @click="addCurrentRune"></el-button>
      </el-tooltip>

      <el-tooltip content="Remove all custom rune for champion.">
        <el-popconfirm :icon="Warning" icon-color="red" title="Sure?" @confirm="removeAllRune">
          <template #reference>
            <el-button type="danger" style="margin-bottom: 10px" :icon="Delete" class="nearLeft" circle></el-button>
          </template>
        </el-popconfirm>

      </el-tooltip>
    </el-row>

    <AmumuLoading v-if="!loading && build?.runes?.length == 0">
      <h4>It seems that the current champion is not suitable for this lane.</h4>
    </AmumuLoading>

    <el-skeleton v-else :rows="15" style="margin-top: 15px;" :loading="loading" animated>
      <el-scrollbar :height="scrollbarHeight">
        <el-row align="middle" justify="center">
          <el-tag effect="dark" round class="nearLeft" :type="build?.is_cache ? 'info' : 'success'">
            {{ build?.is_cache ? "Cache" : "Online" }}</el-tag>
          <el-tag round :type="build?.is_cache ? 'info' : 'success'" class="nearLeft" v-if="wasteTime > 0">
            {{ wasteTime.toFixed(2) }}s</el-tag>
        </el-row>

        <Spells v-if="build && championInfo" :spells="build.spells"></Spells>
        <Runes v-if="build && championInfo" :is-custom="build.is_custom" :runes="build.runes"
          :champion-name="championInfo.name" :selected-lane="selectedLane" :game-mode="gameMode"></Runes>
      </el-scrollbar>
    </el-skeleton>
  </div>
</template>

<style>
.runeOrSpell {
  background-color: black !important;
}
</style>