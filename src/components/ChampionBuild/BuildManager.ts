import { Build } from "../models/Backend/SelectChampion";
import { lanes } from "../utils/global";
import { getChampionAllBuild, getChampionBuild } from "../utils/lcu";

export class BuildManager {
    builds: Map<string, Build> = new Map();

    private getCacheKeyWith(championId: string, lane: string, gameMode: string): string {
        return `${championId}.${lane}.${gameMode}`
    }

    public getBuildFromCache(championId: string, lane: string, gameMode: string): Build | undefined {
        return this.builds.get(this.getCacheKeyWith(championId, lane, gameMode))
    }

    public async getBuildFromCacheElseRequest(championId: string, lane: string, gameMode: string): Promise<Build> {
        let key = this.getCacheKeyWith(championId, lane, gameMode)
        let build = this.builds.get(key) ?? await this.getBuildFromRequest(championId, lane, gameMode)
        return build
    }

    public async getBuildFromRequest(championId: string, lane: string, gameMode: string): Promise<Build> {
        let key = this.getCacheKeyWith(championId, lane, gameMode)
        let build = await getChampionBuild(championId, lane, gameMode)
        this.builds.set(key, build)

        return build
    }

    public async getPopularBuild(championId: string, gameMode: string): Promise<Build | undefined> {
        let allBuild = await getChampionAllBuild(championId, gameMode);
        let best: Build | undefined = undefined;
        for (let index = 0; index < allBuild.length; index++) {
            const build = allBuild[index];
            this.updateBuild(championId, gameMode, build);
            if (build.runes.length > 0) {
                if (!best || build.runes[0].play > best.runes[0].play) {
                    best = build;
                }
            }
        }
        return best
    }

    public updateBuild(championId: string, gameMode: string, build: Build) {
        let key = this.getCacheKeyWith(championId, build.lane, gameMode)
        this.builds.set(key, build)
    }
}