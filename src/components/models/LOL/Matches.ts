export interface Matches {
    accountId: number,
    games: {
        gameBeginDate: string,
        gameCount: number,
        gameEndDate: string,
        gameIndexBegin: number,
        gameIndexEnd: number,
        games: [
            Game
        ]
    },
    platformId: string
}

export interface Game {
    gameCreation: number,
    gameCreationDate: string,
    gameDuration: number,
    gameId: number,
    gameMode: string,
    gameType: string,
    gameVersion: string,
    mapId: number,
    participantIdentities: [
        ParticipantIdentity
    ],
    participants: [
        Participant
    ],
    platformId: string,
    queueId: number,
    seasonId: number,
    teams: [
        {
            bans: [
                {
                    championId: number,
                    pickTurn: number
                }
            ],
            baronKills: number,
            dominionVictoryScore: number,
            dragonKills: number,
            firstBaron: boolean,
            firstBlood: boolean,
            firstDargon: boolean,
            firstInhibitor: boolean,
            firstTower: boolean,
            inhibitorKills: number,
            riftHeraldKills: number,
            teamId: number,
            towerKills: number,
            vilemawKills: number,
            win: string
        }
    ]
}

export interface ParticipantIdentity {
    participantId: number,
    player: {
        accountId: number,
        currentAccountId: number,
        currentPlatformId: string,
        matchHistoryUri: string,
        platformId: string,
        profileIcon: number,
        summonerId: number,
        summonerName: string
    }
}

export interface Participant {
    championId: number,
    highestAchievedSeasonTier: string,
    participantId: number,
    spell1Id: number,
    spell2Id: number,
    stats: {
        assists: number,
        causedEarlySurrender: boolean,
        champLevel: number,
        combatPlayerScore: number,
        damageDealtToObjectives: number,
        damageDealtToTurrets: number,
        damageSelfMitigated: number,
        deaths: number,
        doubleKills: number,
        earlySurrenderAccomplice: boolean,
        firstBloodAssist: boolean,
        firstBloodKill: boolean,
        firstInhibitorAssist: boolean,
        firstInhibitorKill: boolean,
        firstTowerAssist: boolean,
        firstTowerKill: boolean,
        gameEndedInEarlySurrender: boolean,
        gameEndedInSurrender: boolean,
        goldEarned: number,
        goldSpent: number,
        inhibitorKills: number,
        itemnumber: number,
        item1: number,
        item2: number,
        item3: number,
        item4: number,
        item5: number,
        item6: number,
        killingSprees: number,
        kills: number,
        largestCriticalStrike: number,
        largestKillingSpree: number,
        largestMultiKill: number,
        longestTimeSpentLiving: number,
        magicDamageDealt: number,
        magicDamageDealtToChampions: number,
        magicalDamageTaken: number,
        neutralMinionsKilled: number,
        neutralMinionsKilledEnemyJungle: number,
        neutralMinionsKilledTeamJungle: number,
        objectivePlayerScore: number,
        participantId: number,
        pentaKills: number,
        perknumber: number,
        perknumberVar1: number,
        perknumberVar2: number,
        perknumberVar3: number,
        perk1: number,
        perk1Var1: number,
        perk1Var2: number,
        perk1Var3: number,
        perk2: number,
        perk2Var1: number,
        perk2Var2: number,
        perk2Var3: number,
        perk3: number,
        perk3Var1: number,
        perk3Var2: number,
        perk3Var3: number,
        perk4: number,
        perk4Var1: number,
        perk4Var2: number,
        perk4Var3: number,
        perk5: number,
        perk5Var1: number,
        perk5Var2: number,
        perk5Var3: number,
        perkPrimaryStyle: number,
        perkSubStyle: number,
        physicalDamageDealt: number,
        physicalDamageDealtToChampions: number,
        physicalDamageTaken: number,
        playerScorenumber: number,
        playerScore1: number,
        playerScore2: number,
        playerScore3: number,
        playerScore4: number,
        playerScore5: number,
        playerScore6: number,
        playerScore7: number,
        playerScore8: number,
        playerScore9: number,
        quadraKills: number,
        sightWardsBoughtInGame: number,
        teamEarlySurrendered: boolean,
        timeCCingOthers: number,
        totalDamageDealt: number,
        totalDamageDealtToChampions: number,
        totalDamageTaken: number,
        totalHeal: number,
        totalMinionsKilled: number,
        totalPlayerScore: number,
        totalScoreRank: number,
        totalTimeCrowdControlDealt: number,
        totalUnitsHealed: number,
        tripleKills: number,
        booleanDamageDealt: number,
        booleanDamageDealtToChampions: number,
        booleanDamageTaken: number,
        turretKills: number,
        unrealKills: number,
        visionScore: number,
        visionWardsBoughtInGame: number,
        wardsKilled: number,
        wardsPlaced: number,
        win: boolean
    },
    teamId: number,
    timeline: {
        creepsPerMinDeltas: {
            additionalProp1: number,
            additionalProp2: number,
            additionalProp3: number
        },
        csDiffPerMinDeltas: {
            additionalProp1: number,
            additionalProp2: number,
            additionalProp3: number
        },
        damageTakenDiffPerMinDeltas: {
            additionalProp1: number,
            additionalProp2: number,
            additionalProp3: number
        },
        damageTakenPerMinDeltas: {
            additionalProp1: number,
            additionalProp2: number,
            additionalProp3: number
        },
        goldPerMinDeltas: {
            additionalProp1: number,
            additionalProp2: number,
            additionalProp3: number
        },
        lane: string,
        participantId: number,
        role: string,
        xpDiffPerMinDeltas: {
            additionalProp1: number,
            additionalProp2: number,
            additionalProp3: number
        },
        xpPerMinDeltas: {
            additionalProp1: number,
            additionalProp2: number,
            additionalProp3: number
        }
    }
}