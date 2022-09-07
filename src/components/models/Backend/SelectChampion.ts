export interface RuneItem {
    id: number,
    primary_page_id: number,
    primary_rune_ids: number[],
    secondary_page_id: number,
    secondary_rune_ids: number[],
    stat_mod_ids: number[],
    play: number,
    win: number,
    pick_rate: number,
    name: string,
}

export interface SpellItem {
    ids: number[],
    play: number,
    win: number,
    pick_rate: number,
}

export interface Build {
    runes: RuneItem[],
    is_custom: boolean,
    spells: SpellItem[],
    is_cache: boolean,
    lane: string,
}