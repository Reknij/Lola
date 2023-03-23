import { invoke } from "@tauri-apps/api";
import { LolRuneItem } from "../models/LOL/LolRuneItem";
import { Build, RuneItem, SpellItem } from "../models/Backend/SelectChampion";
import { GameMode } from "../models/LOL/gameMode";
import { SummonerInfo } from "../models/LOL/SummonerInfo";
import { currentSummoner } from "./global";
import { ChampionInfo } from "../models/LOL/ChampionInfo";

export function lget<T>(url: string): Promise<T> {
    return invoke<T>("lcu_get", {
        url,
    });
}

export function lpost<T>(url: string, body: any): Promise<T> {
    return invoke<T>("lcu_post", {
        url,
        body
    });
}

export function lput<T>(url: string, body: any): Promise<T> {
    return invoke<T>("lcu_put", {
        url,
        body
    });
}

export function lpatch<T>(url: string, body: any): Promise<T> {
    return invoke<T>("lcu_patch", {
        url,
        body
    });
}

export function ldelete<T>(url: string): Promise<T> {
    return invoke<T>("lcu_delete", {
        url,
    });
}

export async function getChampionIconUrl(championId: string): Promise<string> {
    let icon = await invoke<string>("get_champion_icon", {
        championId: championId
    })
    return icon;
}

export async function isChampionSelecting(): Promise<boolean> {
    return (await lget<string>("/lol-gameflow/v1/gameflow-phase")) == "ChampSelect";
}

export async function getChampionRawInfo(championKey: string): Promise<ChampionInfo> {
    return await invoke("get_champion_raw_info", {
        championKey
    });
}

export async function getCurrentRune(): Promise<LolRuneItem | undefined> {
    const list = await lget(`lol-perks/v1/pages`) as Array<LolRuneItem>;
    const current = list.find((i) => i.current && i.isDeletable);
    if (!current) {
        invoke("log", {
            msg: "Can't get current rune page!"
        })
        return;
    }
    invoke("log", {
        msg: `current id is ${current.id}`
    })
    return current
}

export async function setCurrentRune(runeItem: RuneItem) {
    let current = await getCurrentRune();
    if (!current) {
        console.log("get current rune null.")
        return;
    }
    await ldelete(`lol-perks/v1/pages/${current.id}`); //delete rune
    toLolRuneItem(current, runeItem);
    let a = await lpost(`lol-perks/v1/pages`, current).catch(e => invoke("log", { msg: e }))
}

export async function setCurrentSpell(spellItem: SpellItem) {
    await lpatch('/lol-champ-select/v1/session/my-selection', {
        spell1Id: spellItem.ids[0],
        spell2Id: spellItem.ids[1],
    })
}

export async function getChampionBuild(championId: string, lane: string, gameMode: string): Promise<Build> {
    return await invoke<Build>("get_champion_build", {
        championId,
        lane,
        gameMode,
      })
}

export async function getChampionAllBuild(championId: string, gameMode: string): Promise<Build[]> {
    return await invoke<Build[]>("get_champion_all_build", {
        championId,
        gameMode,
      });
}

export async function addChampionCustomRune(championId: string, gameMode: string, runeItem: LolRuneItem): Promise<boolean> {
    return await invoke("add_champion_custom_rune", {
        championId,
        gameMode,
        runeItem
    });
}

export async function removeChampionCustomRune(championId: string, gameMode: string, runeName: string): Promise<boolean> {
    return await invoke("remove_champion_custom_rune", {
        championId,
        gameMode,
        runeName
    });
}

export async function removeChampionCustomRunes(championId: string, gameMode: string): Promise<boolean> {
    return await invoke("remove_champion_custom_runes", {
        championId,
        gameMode
    });
}

export async function getCurrentGameMode(): Promise<GameMode> {
    let session = await lget<any>('/lol-gameflow/v1/session');
    if (session?.gameData?.queue?.gameMode == 'ARAM') return GameMode.aram
    else if (session?.gameData?.queue?.gameMode == 'URF') return GameMode.urf
    else if (session?.gameData?.queue?.gameMode == 'CLASSIC') return GameMode.classic
    else return GameMode.unknown;
}

export async function getChampionKeyFromSession(session: any): Promise<number> {
    if (session.actions) {
        for (let action of session.actions) {
            for (let e of action) {
                if (e.actorCellId == session.localPlayerCellId && e.isInProgress && e.type != 'ban' && e.championId) {
                    return e.championId;
                }
            }
        }
    }

    let championId = await lget<number>('/lol-champ-select/v1/current-champion');
    if (isNaN(championId)) return 0;
    return championId;
}

function toLolRuneItem(original: LolRuneItem, r: RuneItem): any {
    original.name = r.name;
    original.primaryStyleId = r.primary_page_id;
    original.subStyleId = r.secondary_page_id;
    original.selectedPerkIds = r.primary_rune_ids.concat(r.secondary_rune_ids, r.stat_mod_ids);
}

export function getAssignedPositionFromSession(session: any): string {
    for (let index = 0; index < session.myTeam.length; index++) {
        const summoner = session.myTeam[index];
        if (summoner.summonerId == currentSummoner.value?.summonerId) {
            let lane = (summoner.assignedPosition as string).toLowerCase();
            switch (lane) {
                case 'bottom':
                    return 'bot';
                case 'jungle':
                    return 'jungle';
                case 'top':
                    return 'top';
                case 'middle':
                    return 'mid';
                case 'utility':
                    return 'support';
                default:
                    return 'none';
            }
        }
    }

    return 'none'
}

export function getGameName(queueId: number): string {
    switch (queueId) {
        case 420: return 'RANKED SOLO/DUO';
        case 430: return 'BLIND PICK';
        case 440: return 'RANKED FLEX';
        case 450: return 'ARAM';
        case 900: return 'URF';
    }
    return 'Other Mode'
}