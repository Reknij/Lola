import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import { SummonerInfo } from "../models/LOL/SummonerInfo";

let summonerIconUrl = ref("");
let currentSummoner = ref<SummonerInfo>();
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
      label: "Bot",
      value: "bot",
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

async function showInFolder(path: string) {
  return await invoke("show_in_folder", {
    path
});
}

export {
    currentSummoner,
    summonerIconUrl,
    lanes,
    showInFolder,
}