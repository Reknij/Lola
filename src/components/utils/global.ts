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

export {
    currentSummoner,
    summonerIconUrl,
    lanes,
}