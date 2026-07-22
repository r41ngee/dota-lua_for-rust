use super::entity::CBaseEntity;
use super::npc::{BaseNpc, BaseNpcHero, CDOTAPlayer};
use super::vector::Vector;

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CCustomGameEventManager;

impl CCustomGameEventManager {
    pub fn RegisterListener(&self, event_name: String, handler: crate::types::Function) -> i32 { unimplemented!() }
    pub fn Send_ServerToAllClients(&self, event_name: String, event_data: crate::types::Table) { unimplemented!() }
    pub fn Send_ServerToPlayer(&self, player: CDOTAPlayer, event_name: String, event_data: crate::types::Table) { unimplemented!() }
    pub fn Send_ServerToTeam(&self, team: i32, event_name: String, event_data: crate::types::Table) { unimplemented!() }
    pub fn UnregisterListener(&self, listener: i32) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CCustomNetTableManager;

impl CCustomNetTableManager {
    pub fn GetTableValue(&self, arg1: String, arg2: String) -> crate::types::Table { unimplemented!() }
    pub fn SetTableValue(&self, arg1: String, arg2: String, arg3: crate::types::Table) -> bool { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CDOTABaseGameMode;

impl CDOTABaseGameMode {
    pub fn AreWeatherEffectsDisabled(&self) -> bool { unimplemented!() }
    pub fn ClearBountyRunePickupFilter(&self) { unimplemented!() }
    pub fn ClearDamageFilter(&self) { unimplemented!() }
    pub fn ClearExecuteOrderFilter(&self) { unimplemented!() }
    pub fn ClearHealingFilter(&self) { unimplemented!() }
    pub fn ClearItemAddedToInventoryFilter(&self) { unimplemented!() }
    pub fn ClearModifierGainedFilter(&self) { unimplemented!() }
    pub fn ClearModifyExperienceFilter(&self) { unimplemented!() }
    pub fn ClearModifyGoldFilter(&self) { unimplemented!() }
    pub fn ClearRuneSpawnFilter(&self) { unimplemented!() }
    pub fn ClearTrackingProjectileFilter(&self) { unimplemented!() }
    pub fn DisableHudFlip(&self, b_disable: bool) { unimplemented!() }
    pub fn GetAlwaysShowPlayerInventory(&self) -> bool { unimplemented!() }
    pub fn GetAlwaysShowPlayerNames(&self) -> bool { unimplemented!() }
    pub fn GetAnnouncerDisabled(&self) -> bool { unimplemented!() }
    pub fn GetCameraDistanceOverride(&self) -> f64 { unimplemented!() }
    pub fn GetCustomAttributeDerivedStatValue(&self, n_derived_stat_type: i32) -> f64 { unimplemented!() }
    pub fn GetCustomBuybackCooldownEnabled(&self) -> bool { unimplemented!() }
    pub fn GetCustomBuybackCostEnabled(&self) -> bool { unimplemented!() }
    pub fn GetCustomHeroMaxLevel(&self) -> i32 { unimplemented!() }
    pub fn GetFixedRespawnTime(&self) -> f64 { unimplemented!() }
    pub fn GetFogOfWarDisabled(&self) -> bool { unimplemented!() }
    pub fn GetGoldSoundDisabled(&self) -> bool { unimplemented!() }
    pub fn GetHUDVisible(&self, i_element: i32) -> bool { unimplemented!() }
    pub fn GetMaximumAttackSpeed(&self) -> i32 { unimplemented!() }
    pub fn GetMinimumAttackSpeed(&self) -> i32 { unimplemented!() }
    pub fn GetRecommendedItemsDisabled(&self) -> bool { unimplemented!() }
    pub fn GetRespawnTimeScale(&self) -> f64 { unimplemented!() }
    pub fn GetStashPurchasingDisabled(&self) -> bool { unimplemented!() }
    pub fn GetStickyItemDisabled(&self) -> bool { unimplemented!() }
    pub fn GetTopBarTeamValuesOverride(&self) -> bool { unimplemented!() }
    pub fn GetTopBarTeamValuesVisible(&self) -> bool { unimplemented!() }
    pub fn GetTowerBackdoorProtectionEnabled(&self) -> bool { unimplemented!() }
    pub fn GetUseCustomHeroLevels(&self) -> bool { unimplemented!() }
    pub fn IsBuybackEnabled(&self) -> bool { unimplemented!() }
    pub fn IsDaynightCycleDisabled(&self) -> bool { unimplemented!() }
    pub fn SetAbilityTuningValueFilter(&self, filter_func: crate::types::Function, h_context: crate::types::Any) { unimplemented!() }
    pub fn SetAlwaysShowPlayerInventory(&self, b_always_show: bool) { unimplemented!() }
    pub fn SetAlwaysShowPlayerNames(&self, b_enabled: bool) { unimplemented!() }
    pub fn SetAnnouncerDisabled(&self, b_disabled: bool) { unimplemented!() }
    pub fn SetBotThinkingEnabled(&self, b_enabled: bool) { unimplemented!() }
    pub fn SetBotsAlwaysPushWithHuman(&self, b_always_push: bool) { unimplemented!() }
    pub fn SetBotsInLateGame(&self, b_late_game: bool) { unimplemented!() }
    pub fn SetBotsMaxPushTier(&self, n_max_tier: i32) { unimplemented!() }
    pub fn SetBountyRunePickupFilter(&self, filter_func: crate::types::Function, h_context: crate::types::Any) { unimplemented!() }
    pub fn SetBuybackEnabled(&self, b_enabled: bool) { unimplemented!() }
    pub fn SetCameraDistanceOverride(&self, fl_camera_distance_override: f64) { unimplemented!() }
    pub fn SetCameraSmoothCountOverride(&self, n_smooth_count: i32) { unimplemented!() }
    pub fn SetCustomAttributeDerivedStatValue(&self, n_stat_type: i32, fl_new_value: f64) { unimplemented!() }
    pub fn SetCustomBuybackCooldownEnabled(&self, b_enabled: bool) { unimplemented!() }
    pub fn SetCustomBuybackCostEnabled(&self, b_enabled: bool) { unimplemented!() }
    pub fn SetCustomGameForceHero(&self, p_hero_name: String) { unimplemented!() }
    pub fn SetCustomHeroMaxLevel(&self, i_max_level: i32) { unimplemented!() }
    pub fn SetCustomTerrainWeatherEffect(&self, psz_effect_name: String) { unimplemented!() }
    pub fn SetCustomXPRequiredToReachNextLevel(&self, h_table: crate::types::Table) { unimplemented!() }
    pub fn SetDamageFilter(&self, filter_func: crate::types::Function, h_context: crate::types::Any) { unimplemented!() }
    pub fn SetDaynightCycleDisabled(&self, b_disable: bool) { unimplemented!() }
    pub fn SetDeathOverlayDisabled(&self, b_disabled: bool) { unimplemented!() }
    pub fn SetExecuteOrderFilter(&self, filter_func: crate::types::Function, h_context: crate::types::Any) { unimplemented!() }
    pub fn SetFixedRespawnTime(&self, fl_fixed_respawn_time: f64) { unimplemented!() }
    pub fn SetFogOfWarDisabled(&self, b_disabled: bool) { unimplemented!() }
    pub fn SetFountainConstantManaRegen(&self, fl_constant_mana_regen: f64) { unimplemented!() }
    pub fn SetFountainPercentageHealthRegen(&self, fl_percentage_health_regen: f64) { unimplemented!() }
    pub fn SetFountainPercentageManaRegen(&self, fl_percentage_mana_regen: f64) { unimplemented!() }
    pub fn SetFriendlyBuildingMoveToEnabled(&self, b_enabled: bool) { unimplemented!() }
    pub fn SetGoldSoundDisabled(&self, b_disabled: bool) { unimplemented!() }
    pub fn SetHUDVisible(&self, i_hud_element: i32, b_visible: bool) { unimplemented!() }
    pub fn SetHealingFilter(&self, h_function: crate::types::Function, h_context: crate::types::Any) { unimplemented!() }
    pub fn SetHudCombatEventsDisabled(&self, b_disabled: bool) { unimplemented!() }
    pub fn SetItemAddedToInventoryFilter(&self, filter_func: crate::types::Function, h_context: crate::types::Any) { unimplemented!() }
    pub fn SetKillingSpreeAnnouncerDisabled(&self, b_disabled: bool) { unimplemented!() }
    pub fn SetLoseGoldOnDeath(&self, b_enabled: bool) { unimplemented!() }
    pub fn SetMaximumAttackSpeed(&self, n_max_speed: i32) { unimplemented!() }
    pub fn SetMinimumAttackSpeed(&self, n_min_speed: i32) { unimplemented!() }
    pub fn SetModifierGainedFilter(&self, filter_func: crate::types::Function, h_context: crate::types::Any) { unimplemented!() }
    pub fn SetModifyExperienceFilter(&self, filter_func: crate::types::Function, h_context: crate::types::Any) { unimplemented!() }
    pub fn SetModifyGoldFilter(&self, filter_func: crate::types::Function, h_context: crate::types::Any) { unimplemented!() }
    pub fn SetOverrideSelectionEntity(&self, h_override_entity: BaseNpc) { unimplemented!() }
    pub fn SetRecommendedItemsDisabled(&self, b_disabled: bool) { unimplemented!() }
    pub fn SetRemoveIllusionsOnDeath(&self, b_remove: bool) { unimplemented!() }
    pub fn SetRespawnTimeScale(&self, fl_value: f64) { unimplemented!() }
    pub fn SetRuneEnabled(&self, n_rune: i32, b_enabled: bool) { unimplemented!() }
    pub fn SetRuneSpawnFilter(&self, filter_func: crate::types::Function, h_context: crate::types::Any) { unimplemented!() }
    pub fn SetSelectionGoldPenaltyEnabled(&self, b_enabled: bool) { unimplemented!() }
    pub fn SetStashPurchasingDisabled(&self, b_disabled: bool) { unimplemented!() }
    pub fn SetStickyItemDisabled(&self, b_disabled: bool) { unimplemented!() }
    pub fn SetTopBarTeamValue(&self, i_team: i32, n_value: i32) { unimplemented!() }
    pub fn SetTopBarTeamValuesOverride(&self, b_override: bool) { unimplemented!() }
    pub fn SetTopBarTeamValuesVisible(&self, b_visible: bool) { unimplemented!() }
    pub fn SetTowerBackdoorProtectionEnabled(&self, b_enabled: bool) { unimplemented!() }
    pub fn SetTrackingProjectileFilter(&self, filter_func: crate::types::Function, h_context: crate::types::Any) { unimplemented!() }
    pub fn SetUnseenFogOfWarEnabled(&self, b_enabled: bool) { unimplemented!() }
    pub fn SetUseCustomHeroLevels(&self, b_enabled: bool) { unimplemented!() }
    pub fn SetWeatherEffectsDisabled(&self, b_disable: bool) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CDOTAGameManager;

impl CDOTAGameManager {
    pub fn GetHeroDataByName_Script(&self, arg1: String) -> crate::types::Any { unimplemented!() }
    pub fn GetHeroIDByName(&self, arg1: String) -> i32 { unimplemented!() }
    pub fn GetHeroNameByID(&self, arg1: i32) -> String { unimplemented!() }
    pub fn GetHeroNameForUnitName(&self, arg1: String) -> String { unimplemented!() }
    pub fn GetHeroUnitNameByID(&self, arg1: i32) -> String { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CDOTAGamerules;

impl CDOTAGamerules {
    pub fn AddEventMetadataLeaderboardEntry(&self, arg1: String, arg2: i32, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: i32, arg9: i32) -> bool { unimplemented!() }
    pub fn AddMinimapDebugPoint(&self, arg1: i32, arg2: Vector, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: i32) { unimplemented!() }
    pub fn AddMinimapDebugPointForTeam(&self, arg1: i32, arg2: Vector, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: i32) { unimplemented!() }
    pub fn BeginNightstalkerNight(&self, duration: f64) { unimplemented!() }
    pub fn BeginTemporaryNight(&self, duration: f64) { unimplemented!() }
    pub fn Defeated(&self) { unimplemented!() }
    pub fn DidMatchSignoutTimeOut(&self) -> bool { unimplemented!() }
    pub fn EnableCustomGameSetupAutoLaunch(&self, enabled: bool) { unimplemented!() }
    pub fn FinishCustomGameSetup(&self) { unimplemented!() }
    pub fn GetCustomGameDifficulty(&self) -> i32 { unimplemented!() }
    pub fn GetCustomGameTeamMaxPlayers(&self, team: i32) -> i32 { unimplemented!() }
    pub fn GetDOTATime(&self, include_pre_game: bool, include_negative_time: bool) -> f64 { unimplemented!() }
    pub fn GetDifficulty(&self) -> i32 { unimplemented!() }
    pub fn GetDroppedItem(&self, index: i32) -> super::item::CBaseItem { unimplemented!() }
    pub fn GetGameFrameTime(&self) -> f64 { unimplemented!() }
    pub fn GetGameModeEntity(&self) -> CDOTABaseGameMode { unimplemented!() }
    pub fn GetGameSessionConfigValue(&self, arg1: String, arg2: String) -> String { unimplemented!() }
    pub fn GetGameTime(&self) -> f64 { unimplemented!() }
    pub fn GetMatchID(&self) -> i64 { unimplemented!() }
    pub fn GetMatchSignoutComplete(&self) -> bool { unimplemented!() }
    pub fn GetNianTotalDamageTaken(&self) -> i32 { unimplemented!() }
    pub fn GetPlayerCustomGameAccountRecord(&self, arg1: i32) -> crate::types::Table { unimplemented!() }
    pub fn GetTimeOfDay(&self) -> f64 { unimplemented!() }
    pub fn IsCheatMode(&self) -> bool { unimplemented!() }
    pub fn IsDaytime(&self) -> bool { unimplemented!() }
    pub fn IsGamePaused(&self) -> bool { unimplemented!() }
    pub fn IsHeroRespawnEnabled(&self) -> bool { unimplemented!() }
    pub fn IsNightstalkerNight(&self) -> bool { unimplemented!() }
    pub fn IsTemporaryNight(&self) -> bool { unimplemented!() }
    pub fn LockCustomGameSetupTeamAssignment(&self, locked: bool) { unimplemented!() }
    pub fn MakeTeamLose(&self, team: i32) { unimplemented!() }
    pub fn NumDroppedItems(&self) -> i32 { unimplemented!() }
    pub fn PlayerHasCustomGameHostPrivileges(&self, player: CDOTAPlayer) -> bool { unimplemented!() }
    pub fn Playtesting_UpdateAddOnKeyValues(&self) { unimplemented!() }
    pub fn ResetDefeated(&self) { unimplemented!() }
    pub fn ResetToHeroSelection(&self) { unimplemented!() }
    pub fn SendCustomMessage(&self, arg1: String, arg2: i32, arg3: i32) { unimplemented!() }
    pub fn SendCustomMessageToTeam(&self, arg1: String, arg2: i32, arg3: i32, arg4: i32) { unimplemented!() }
    pub fn SetCreepMinimapIconScale(&self, scale: f64) { unimplemented!() }
    pub fn SetCustomGameAccountRecordSaveFunction(&self, arg1: crate::types::Any, arg2: crate::types::Any) { unimplemented!() }
    pub fn SetCustomGameAllowBattleMusic(&self, allow: bool) { unimplemented!() }
    pub fn SetCustomGameAllowHeroPickMusic(&self, allow: bool) { unimplemented!() }
    pub fn SetCustomGameAllowMusicAtGameStart(&self, allow: bool) { unimplemented!() }
    pub fn SetCustomGameDifficulty(&self, difficulty: i32) { unimplemented!() }
    pub fn SetCustomGameEndDelay(&self, delay: f64) { unimplemented!() }
    pub fn SetCustomGameSetupAutoLaunchDelay(&self, delay: f64) { unimplemented!() }
    pub fn SetCustomGameSetupRemainingTime(&self, remaining_time: f64) { unimplemented!() }
    pub fn SetCustomGameSetupTimeout(&self, timeout: f64) { unimplemented!() }
    pub fn SetCustomGameTeamMaxPlayers(&self, team: i32, max_players: i32) { unimplemented!() }
    pub fn SetCustomVictoryMessage(&self, message: String) { unimplemented!() }
    pub fn SetCustomVictoryMessageDuration(&self, duration: f64) { unimplemented!() }
    pub fn SetEventMetadataCustomTable(&self, arg1: crate::types::Table) -> bool { unimplemented!() }
    pub fn SetEventSignoutCustomTable(&self, arg1: crate::types::Table) -> bool { unimplemented!() }
    pub fn SetFirstBloodActive(&self, active: bool) { unimplemented!() }
    pub fn SetGameWinner(&self, team: i32) { unimplemented!() }
    pub fn SetGoldPerTick(&self, amount: i32) { unimplemented!() }
    pub fn SetGoldTickTime(&self, time: f64) { unimplemented!() }
    pub fn SetHeroMinimapIconScale(&self, scale: f64) { unimplemented!() }
    pub fn SetHeroRespawnEnabled(&self, enabled: bool) { unimplemented!() }
    pub fn SetHeroSelectionTime(&self, selection_time: f64) { unimplemented!() }
    pub fn SetHideKillMessageHeaders(&self, hide_headers: bool) { unimplemented!() }
    pub fn SetOverlayHealthBarUnit(&self, unit: BaseNpc, style: i32) { unimplemented!() }
    pub fn SetPostGameTime(&self, time: f64) { unimplemented!() }
    pub fn SetPreGameTime(&self, time: f64) { unimplemented!() }
    pub fn SetRuneMinimapIconScale(&self, fl_rune_minimap_icon_scale: f64) { unimplemented!() }
    pub fn SetRuneSpawnTime(&self, time: f64) { unimplemented!() }
    pub fn SetSafeToLeave(&self, b_safe_to_leave: bool) { unimplemented!() }
    pub fn SetSameHeroSelectionEnabled(&self, enabled: bool) { unimplemented!() }
    pub fn SetShowcaseTime(&self, time: f64) { unimplemented!() }
    pub fn SetStartingGold(&self, amount: i32) { unimplemented!() }
    pub fn SetStrategyTime(&self, time: f64) { unimplemented!() }
    pub fn SetTimeOfDay(&self, time: f64) { unimplemented!() }
    pub fn SetTreeRegrowTime(&self, time: f64) { unimplemented!() }
    pub fn SetUseBaseGoldBountyOnHeroes(&self, use_base_gold_bounties: bool) { unimplemented!() }
    pub fn SetUseCustomHeroXPValues(&self, use_custom_xp_values: bool) { unimplemented!() }
    pub fn SetUseUniversalShopMode(&self, use_universal_shop_mode: bool) { unimplemented!() }
    pub fn State_Get(&self) -> i32 { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CDOTATutorial;

impl CDOTATutorial {
    pub fn AddBot(&self, hero_name: String, unknown1: String, unknown2: String, unknown3: bool) -> bool { unimplemented!() }
    pub fn AddQuest(&self, arg1: String, arg2: i32, arg3: String, arg4: String) { unimplemented!() }
    pub fn AddShopWhitelistItem(&self, item_name: String) { unimplemented!() }
    pub fn CompleteQuest(&self, arg1: String) { unimplemented!() }
    pub fn CreateLocationTask(&self, arg1: Vector) { unimplemented!() }
    pub fn EnableCreepAggroViz(&self, arg1: bool) { unimplemented!() }
    pub fn EnablePlayerOffscreenTip(&self, arg1: bool) { unimplemented!() }
    pub fn EnableTowerAggroViz(&self, arg1: bool) { unimplemented!() }
    pub fn FinishTutorial(&self) { unimplemented!() }
    pub fn ForceGameStart(&self) { unimplemented!() }
    pub fn GetTimeFrozen(&self) -> bool { unimplemented!() }
    pub fn IsItemInWhiteList(&self, item_name: String) -> bool { unimplemented!() }
    pub fn RemoveShopWhitelistItem(&self, item_name: String) { unimplemented!() }
    pub fn SelectHero(&self, hero_name: String) { unimplemented!() }
    pub fn SelectPlayerTeam(&self, arg1: String) { unimplemented!() }
    pub fn SetItemGuide(&self, arg1: String) { unimplemented!() }
    pub fn SetOrModifyPlayerGold(&self, gold_amount: i32, set_not_modify: bool) { unimplemented!() }
    pub fn SetQuickBuy(&self, item_name: String) { unimplemented!() }
    pub fn SetShopOpen(&self, open: bool) { unimplemented!() }
    pub fn SetTimeFrozen(&self, time_frozen: bool) { unimplemented!() }
    pub fn SetTutorialConvar(&self, arg1: String, arg2: String) { unimplemented!() }
    pub fn SetTutorialUI(&self, arg1: i32) { unimplemented!() }
    pub fn SetWhiteListEnabled(&self, whitelist_enabled: bool) { unimplemented!() }
    pub fn StartTutorialMode(&self) { unimplemented!() }
    pub fn UpgradePlayerAbility(&self, ability_name: String) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CDOTACustomUIManager;

impl CDOTACustomUIManager {
    pub fn DynamicHud_Create(&self, arg1: i32, arg2: String, arg3: String, arg4: crate::types::Table) { unimplemented!() }
    pub fn DynamicHud_Destroy(&self, arg1: i32, arg2: String) { unimplemented!() }
    pub fn DynamicHud_SetDialogVariables(&self, arg1: i32, arg2: String, arg3: crate::types::Table) { unimplemented!() }
    pub fn DynamicHud_SetVisible(&self, arg1: i32, arg2: String, arg3: bool) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CDOTAPlayerResource;

impl CDOTAPlayerResource {
    pub fn AddAegisPickup(&self, i_player_id: i32) { unimplemented!() }
    pub fn AddClaimedFarm(&self, i_player_id: i32, fl_farm_value: f64, b_earned_value: bool) { unimplemented!() }
    pub fn AddGoldSpentOnSupport(&self, i_player_id: i32, i_cost: i32) { unimplemented!() }
    pub fn AddRunePickup(&self, i_player_id: i32) { unimplemented!() }
    pub fn AreUnitsSharedWithPlayerID(&self, n_unit_owner_player_id: i32, n_other_player_id: i32) -> bool { unimplemented!() }
    pub fn CanRepick(&self, i_player_id: i32) -> bool { unimplemented!() }
    pub fn ClearKillsMatrix(&self, i_player_id: i32) { unimplemented!() }
    pub fn ClearLastHitMultikill(&self, i_player_id: i32) { unimplemented!() }
    pub fn ClearLastHitStreak(&self, i_player_id: i32) { unimplemented!() }
    pub fn ClearRawPlayerDamageMatrix(&self, i_player_id: i32) { unimplemented!() }
    pub fn ClearStreak(&self, i_player_id: i32) { unimplemented!() }
    pub fn GetAegisPickups(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetAssists(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetBroadcasterChannel(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetBroadcasterChannelSlot(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetClaimedDenies(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetClaimedFarm(&self, i_player_id: i32, b_only_earned: bool) -> f64 { unimplemented!() }
    pub fn GetClaimedMisses(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetConnectionState(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetCreepDamageTaken(&self, i_player_id: i32, b_total: bool) -> i32 { unimplemented!() }
    pub fn GetCustomBuybackCooldown(&self, i_player_id: i32) -> f64 { unimplemented!() }
    pub fn GetCustomBuybackCost(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetCustomTeamAssignment(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetDamageDoneToHero(&self, i_player_id: i32, i_victim_id: i32) -> i32 { unimplemented!() }
    pub fn GetDeaths(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetDenies(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetEventPointsForPlayerID(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetEventPremiumPoints(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetEventRanks(&self, i_player_id: i32) -> crate::types::Any { unimplemented!() }
    pub fn GetGold(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetGoldLostToDeath(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetGoldPerMin(&self, i_player_id: i32) -> f64 { unimplemented!() }
    pub fn GetGoldSpentOnBuybacks(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetGoldSpentOnConsumables(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetGoldSpentOnItems(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetGoldSpentOnSupport(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetHealing(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetHeroDamageTaken(&self, i_player_id: i32, b_total: bool) -> i32 { unimplemented!() }
    pub fn GetKills(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetKillsDoneToHero(&self, i_player_id: i32, i_victim_id: i32) -> i32 { unimplemented!() }
    pub fn GetLastHitMultikill(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetLastHitStreak(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetLastHits(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetLevel(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetMisses(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetNearbyCreepDeaths(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetNthCourierForTeam(&self, n_courier_index: i32, n_team_number: i32) -> super::npc::CDOTAUnitCourier { unimplemented!() }
    pub fn GetNthPlayerIDOnTeam(&self, i_team_number: i32, i_nth_player: i32) -> i32 { unimplemented!() }
    pub fn GetNumConsumablesPurchased(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetNumCouriersForTeam(&self, n_team_number: i32) -> i32 { unimplemented!() }
    pub fn GetNumItemsPurchased(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetPlayer(&self, i_player_id: i32) -> CDOTAPlayer { unimplemented!() }
    pub fn GetPlayerCount(&self) -> i32 { unimplemented!() }
    pub fn GetPlayerCountForTeam(&self, i_team: i32) -> i32 { unimplemented!() }
    pub fn GetPlayerLoadedCompletely(&self, i_player_id: i32) -> bool { unimplemented!() }
    pub fn GetPlayerName(&self, i_player_id: i32) -> String { unimplemented!() }
    pub fn GetRawPlayerDamage(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetReliableGold(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetRespawnSeconds(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetRoshanKills(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetRunePickups(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetSelectedHeroEntity(&self, i_player_id: i32) -> BaseNpcHero { unimplemented!() }
    pub fn GetSelectedHeroID(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetSelectedHeroName(&self, i_player_id: i32) -> String { unimplemented!() }
    pub fn GetSteamAccountID(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetSteamID(&self, i_player_id: i32) -> i64 { unimplemented!() }
    pub fn GetStreak(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetStuns(&self, i_player_id: i32) -> f64 { unimplemented!() }
    pub fn GetTeam(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetTeamKills(&self, i_team: i32) -> i32 { unimplemented!() }
    pub fn GetTeamPlayerCount(&self) -> i32 { unimplemented!() }
    pub fn GetTimeOfLastConsumablePurchase(&self, i_player_id: i32) -> f64 { unimplemented!() }
    pub fn GetTimeOfLastDeath(&self, i_player_id: i32) -> f64 { unimplemented!() }
    pub fn GetTimeOfLastItemPurchase(&self, i_player_id: i32) -> f64 { unimplemented!() }
    pub fn GetTotalEarnedGold(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetTotalEarnedXP(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetTotalGoldSpent(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetTowerDamageTaken(&self, i_player_id: i32, b_total: bool) -> i32 { unimplemented!() }
    pub fn GetTowerKills(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetUnitShareMaskForPlayer(&self, n_player_id: i32, n_other_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetUnreliableGold(&self, i_player_id: i32) -> i32 { unimplemented!() }
    pub fn GetXPPerMin(&self, i_player_id: i32) -> f64 { unimplemented!() }
    pub fn HasCustomGameTicketForPlayerID(&self, i_player_id: i32) -> bool { unimplemented!() }
    pub fn HasRandomed(&self, i_player_id: i32) -> bool { unimplemented!() }
    pub fn HasSelectedHero(&self, i_player_id: i32) -> bool { unimplemented!() }
    pub fn HaveAllPlayersJoined(&self) -> bool { unimplemented!() }
    pub fn IncrementAssists(&self, i_player_id: i32, i_victim_id: i32) { unimplemented!() }
    pub fn IncrementClaimedDenies(&self, i_player_id: i32) { unimplemented!() }
    pub fn IncrementClaimedMisses(&self, i_player_id: i32) { unimplemented!() }
    pub fn IncrementDeaths(&self, i_player_id: i32, i_killer_id: i32) { unimplemented!() }
    pub fn IncrementDenies(&self, i_player_id: i32) { unimplemented!() }
    pub fn IncrementKills(&self, i_player_id: i32, i_victim_id: i32) { unimplemented!() }
    pub fn IncrementLastHitMultikill(&self, i_player_id: i32) { unimplemented!() }
    pub fn IncrementLastHitStreak(&self, i_player_id: i32) { unimplemented!() }
    pub fn IncrementLastHits(&self, i_player_id: i32) { unimplemented!() }
    pub fn IncrementMisses(&self, i_player_id: i32) { unimplemented!() }
    pub fn IncrementNearbyCreepDeaths(&self, i_player_id: i32) { unimplemented!() }
    pub fn IncrementStreak(&self, i_player_id: i32) { unimplemented!() }
    pub fn IncrementTotalEarnedXP(&self, i_player_id: i32, i_xp: i32, n_reason: i32) { unimplemented!() }
    pub fn IsBroadcaster(&self, i_player_id: i32) -> bool { unimplemented!() }
    pub fn IsDisableHelpSetForPlayerID(&self, n_player_id: i32, n_other_player_id: i32) -> bool { unimplemented!() }
    pub fn IsFakeClient(&self, i_player_id: i32) -> bool { unimplemented!() }
    pub fn IsHeroSelected(&self, p_heroname: String) -> bool { unimplemented!() }
    pub fn IsHeroSharedWithPlayerID(&self, n_unit_owner_player_id: i32, n_other_player_id: i32) -> bool { unimplemented!() }
    pub fn IsValidPlayer(&self, i_player_id: i32) -> bool { unimplemented!() }
    pub fn IsValidPlayerID(&self, i_player_id: i32) -> bool { unimplemented!() }
    pub fn IsValidTeamPlayer(&self, i_player_id: i32) -> bool { unimplemented!() }
    pub fn IsValidTeamPlayerID(&self, i_player_id: i32) -> bool { unimplemented!() }
    pub fn ModifyGold(&self, i_player_id: i32, i_gold_change: i32, b_reliable: bool, n_reason: i32) -> i32 { unimplemented!() }
    pub fn ReplaceHeroWith(&self, i_player_id: i32, p_hero_class: String, n_gold: i32, n_xp: i32) -> BaseNpcHero { unimplemented!() }
    pub fn ResetBuybackCostTime(&self, n_player_id: i32) { unimplemented!() }
    pub fn ResetTotalEarnedGold(&self, i_player_id: i32) { unimplemented!() }
    pub fn SetBuybackCooldownTime(&self, n_player_id: i32, fl_buyback_cooldown: f64) { unimplemented!() }
    pub fn SetBuybackGoldLimitTime(&self, n_player_id: i32, fl_buyback_cooldown: f64) { unimplemented!() }
    pub fn SetCameraTarget(&self, n_player_id: i32, h_target: BaseNpc) { unimplemented!() }
    pub fn SetCanRepick(&self, i_player_id: i32, b_can_repick: bool) { unimplemented!() }
    pub fn SetCustomBuybackCooldown(&self, i_player_id: i32, fl_cooldown_time: f64) { unimplemented!() }
    pub fn SetCustomBuybackCost(&self, i_player_id: i32, i_gold_cost: i32) { unimplemented!() }
    pub fn SetCustomPlayerColor(&self, i_player_id: i32, r: i32, g: i32, b: i32) { unimplemented!() }
    pub fn SetCustomTeamAssignment(&self, i_player_id: i32, i_team_assignment: i32) { unimplemented!() }
    pub fn SetGold(&self, i_player_id: i32, i_gold: i32, b_reliable: bool) { unimplemented!() }
    pub fn SetHasRandomed(&self, i_player_id: i32) { unimplemented!() }
    pub fn SetLastBuybackTime(&self, i_player_id: i32, i_last_buyback_time: i32) { unimplemented!() }
    pub fn SetOverrideSelectionEntity(&self, n_player_id: i32, h_entity: BaseNpc) { unimplemented!() }
    pub fn SetUnitShareMaskForPlayer(&self, n_player_id: i32, n_other_player_id: i32, n_flag: i32, b_state: bool) { unimplemented!() }
    pub fn SpendGold(&self, i_player_id: i32, i_cost: i32, i_reason: i32) { unimplemented!() }
    pub fn UpdateTeamSlot(&self, i_player_id: i32, i_team_number: i32, desired_slot: i32) { unimplemented!() }
    pub fn WhoSelectedHero(&self, p_herofilename: String) -> i32 { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CDebugOverlayScriptHelper;

impl CDebugOverlayScriptHelper {
    pub fn Axis(&self, arg1: Vector, arg2: crate::types::QAngle, arg3: f64, arg4: bool, arg5: f64) { unimplemented!() }
    pub fn Box(&self, arg1: Vector, arg2: Vector, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: bool, arg8: f64) { unimplemented!() }
    pub fn BoxAngles(&self, arg1: Vector, arg2: Vector, arg3: Vector, arg4: crate::types::QAngle, arg5: i32, arg6: i32, arg7: i32, arg8: i32, arg9: bool, arg10: f64) { unimplemented!() }
    pub fn Capsule(&self, arg1: Vector, arg2: crate::types::QAngle, arg3: f64, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: i32, arg9: bool, arg10: f64) { unimplemented!() }
    pub fn Circle(&self, arg1: Vector, arg2: crate::types::QAngle, arg3: f64, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: bool, arg9: f64) { unimplemented!() }
    pub fn CircleScreenOriented(&self, arg1: Vector, arg2: f64, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: bool, arg8: f64) { unimplemented!() }
    pub fn Cone(&self, arg1: Vector, arg2: Vector, arg3: f64, arg4: f64, arg5: i32, arg6: i32, arg7: i32, arg8: i32, arg9: bool, arg10: f64) { unimplemented!() }
    pub fn Cross(&self, arg1: Vector, arg2: f64, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: bool, arg8: f64) { unimplemented!() }
    pub fn Cross3D(&self, arg1: Vector, arg2: f64, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: bool, arg8: f64) { unimplemented!() }
    pub fn Cross3DOriented(&self, arg1: Vector, arg2: crate::types::QAngle, arg3: f64, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: bool, arg9: f64) { unimplemented!() }
    pub fn DrawTickMarkedLine(&self, arg1: Vector, arg2: Vector, arg3: f64, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: i32, arg9: bool, arg10: f64) { unimplemented!() }
    pub fn EntityAttachments(&self, arg1: CBaseEntity, arg2: i32, arg3: i32) { unimplemented!() }
    pub fn EntityAxis(&self, arg1: CBaseEntity, arg2: i32, arg3: bool, arg4: f64) { unimplemented!() }
    pub fn EntityBounds(&self, arg1: CBaseEntity, arg2: i32, arg3: i32, arg4: i32, arg5: i32, arg6: bool, arg7: f64) { unimplemented!() }
    pub fn EntitySkeleton(&self, arg1: CBaseEntity, arg2: f64) { unimplemented!() }
    pub fn EntityText(&self, arg1: CBaseEntity, arg2: i32, arg3: String, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: f64) { unimplemented!() }
    pub fn FilledRect2D(&self, arg1: Vector, arg2: Vector, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: i32) { unimplemented!() }
    pub fn HorzArrow(&self, arg1: Vector, arg2: Vector, arg3: f64, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: bool, arg9: f64) { unimplemented!() }
    pub fn Line(&self, arg1: Vector, arg2: Vector, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: bool, arg8: f64) { unimplemented!() }
    pub fn Line2D(&self, arg1: Vector, arg2: Vector, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: f64) { unimplemented!() }
    pub fn PopDebugOverlayScope(&self) { unimplemented!() }
    pub fn PushAndClearDebugOverlayScope(&self, arg1: String) { unimplemented!() }
    pub fn PushDebugOverlayScope(&self, arg1: String) { unimplemented!() }
    pub fn RemoveAllInScope(&self, arg1: String) { unimplemented!() }
    pub fn SolidCone(&self, arg1: Vector, arg2: Vector, arg3: f64, arg4: f64, arg5: i32, arg6: i32, arg7: i32, arg8: i32, arg9: bool, arg10: f64) { unimplemented!() }
    pub fn Sphere(&self, arg1: Vector, arg2: f64, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: bool, arg8: f64) { unimplemented!() }
    pub fn SweptBox(&self, arg1: Vector, arg2: Vector, arg3: Vector, arg4: Vector, arg5: crate::types::QAngle, arg6: i32, arg7: i32, arg8: i32, arg9: i32, arg10: f64) { unimplemented!() }
    pub fn Text(&self, arg1: Vector, arg2: i32, arg3: String, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: i32, arg9: f64) { unimplemented!() }
    pub fn Texture(&self, arg1: String, arg2: Vector, arg3: Vector, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: Vector, arg9: Vector, arg10: f64) { unimplemented!() }
    pub fn Triangle(&self, arg1: Vector, arg2: Vector, arg3: Vector, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: bool, arg9: f64) { unimplemented!() }
    pub fn UnitTestCycleOverlayRenderType(&self) { unimplemented!() }
    pub fn VectorText3D(&self, arg1: Vector, arg2: crate::types::QAngle, arg3: String, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: bool, arg9: f64) { unimplemented!() }
    pub fn VertArrow(&self, arg1: Vector, arg2: Vector, arg3: f64, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: bool, arg9: f64) { unimplemented!() }
    pub fn YawArrow(&self, arg1: Vector, arg2: f64, arg3: f64, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: i32, arg9: bool, arg10: f64) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CEntities;

impl CEntities {
    pub fn CreateByClassname(&self, class_name: String) -> CBaseEntity { unimplemented!() }
    pub fn FindAllByClassname(&self, class_name: String) -> Vec<CBaseEntity> { unimplemented!() }
    pub fn FindAllByClassnameWithin(&self, class_name: String, location: Vector, radius: f64) -> Vec<CBaseEntity> { unimplemented!() }
    pub fn FindAllByModel(&self, model_name: String) -> Vec<CBaseEntity> { unimplemented!() }
    pub fn FindAllByName(&self, name: String) -> Vec<CBaseEntity> { unimplemented!() }
    pub fn FindAllByNameWithin(&self, name: String, location: Vector, radius: f64) -> Vec<CBaseEntity> { unimplemented!() }
    pub fn FindAllByTarget(&self, target: String) -> Vec<CBaseEntity> { unimplemented!() }
    pub fn FindAllInSphere(&self, location: Vector, radius: f64) -> Vec<CBaseEntity> { unimplemented!() }
    pub fn FindByClassname(&self, previous: Option<CBaseEntity>, class_name: String) -> CBaseEntity { unimplemented!() }
    pub fn FindByClassnameNearest(&self, class_name: String, location: Vector, radius: f64) -> CBaseEntity { unimplemented!() }
    pub fn FindByClassnameWithin(&self, previous: Option<CBaseEntity>, class_name: String, location: Vector, radius: f64) -> CBaseEntity { unimplemented!() }
    pub fn FindByModel(&self, previous: Option<CBaseEntity>, model_name: String) -> CBaseEntity { unimplemented!() }
    pub fn FindByModelWithin(&self, previous: Option<CBaseEntity>, model_name: String, location: Vector, radius: f64) -> CBaseEntity { unimplemented!() }
    pub fn FindByName(&self, previous: Option<CBaseEntity>, name: String) -> CBaseEntity { unimplemented!() }
    pub fn FindByNameNearest(&self, name: String, location: Vector, radius: f64) -> CBaseEntity { unimplemented!() }
    pub fn FindByNameWithin(&self, previous: Option<CBaseEntity>, name: String, location: Vector, radius: f64) -> CBaseEntity { unimplemented!() }
    pub fn FindByTarget(&self, previous: Option<CBaseEntity>, target: String) -> CBaseEntity { unimplemented!() }
    pub fn FindInSphere(&self, previous: Option<CBaseEntity>, location: Vector, radius: f64) -> CBaseEntity { unimplemented!() }
    pub fn First(&self) -> CBaseEntity { unimplemented!() }
    pub fn GetLocalPlayer(&self) -> CDOTAPlayer { unimplemented!() }
    pub fn Next(&self, previous: CBaseEntity) -> CBaseEntity { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CScriptHeroList;

impl CScriptHeroList {
    pub fn GetAllHeroes(&self) -> Vec<BaseNpcHero> { unimplemented!() }
    pub fn GetHero(&self, nth: i32) -> BaseNpcHero { unimplemented!() }
    pub fn GetHeroCount(&self) -> i32 { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CScriptParticleManager;

impl CScriptParticleManager {
    pub fn CreateParticle(&self, particle_name: String, particle_attach: i32, owner: Option<BaseNpc>) -> i32 { unimplemented!() }
    pub fn CreateParticleForPlayer(&self, particle_name: String, particle_attach: i32, owner: Option<BaseNpc>, player: CDOTAPlayer) -> i32 { unimplemented!() }
    pub fn CreateParticleForTeam(&self, particle_name: String, particle_attach: i32, owner: Option<BaseNpc>, team: i32) -> i32 { unimplemented!() }
    pub fn DestroyParticle(&self, particle: i32, immediate: bool) { unimplemented!() }
    pub fn GetParticleReplacement(&self, arg1: String, arg2: crate::types::Table) -> String { unimplemented!() }
    pub fn ReleaseParticleIndex(&self, particle: i32) { unimplemented!() }
    pub fn SetParticleAlwaysSimulate(&self, particle: i32) { unimplemented!() }
    pub fn SetParticleControl(&self, particle: i32, control_point: i32, value: Vector) { unimplemented!() }
    pub fn SetParticleControlEnt(&self, particle: i32, control_point: i32, unit: BaseNpc, particle_attach: i32, attachment: String, offset: Vector, lock_orientation: bool) { unimplemented!() }
    pub fn SetParticleControlFallback(&self, arg1: i32, arg2: i32, arg3: Vector) { unimplemented!() }
    pub fn SetParticleControlForward(&self, particle: i32, control_point: i32, forward: Vector) { unimplemented!() }
    pub fn SetParticleControlOrientation(&self, particle: i32, control_point: i32, forward: Vector, right: Vector, up: Vector) { unimplemented!() }
    pub fn SetParticleFoWProperties(&self, arg1: i32, arg2: i32, arg3: i32, arg4: i32) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CConvars;

impl CConvars {
    pub fn GetBool(&self, convar: String) -> bool { unimplemented!() }
    pub fn GetCommandClient(&self) -> CDOTAPlayer { unimplemented!() }
    pub fn GetDOTACommandClient(&self) -> CDOTAPlayer { unimplemented!() }
    pub fn GetFloat(&self, convar: String) -> f64 { unimplemented!() }
    pub fn GetInt(&self, convar: String) -> i32 { unimplemented!() }
    pub fn GetStr(&self, convar: String) -> String { unimplemented!() }
    pub fn RegisterCommand(&self, command_name: String, callback: crate::types::Function, description: String, flags: i32) { unimplemented!() }
    pub fn RegisterConvar(&self, convar_name: String, default_value: String, description: String, flags: i32) { unimplemented!() }
    pub fn SetBool(&self, convar: String, value: bool) { unimplemented!() }
    pub fn SetFloat(&self, convar: String, value: f64) { unimplemented!() }
    pub fn SetInt(&self, convar: String, value: i32) { unimplemented!() }
    pub fn SetStr(&self, convar: String, value: String) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CGridNav;

impl CGridNav {
    pub fn CalculatePathForPos(&self, arg1: Vector, arg2: Vector, arg3: crate::types::Any, arg4: f64, arg5: crate::types::Any) -> crate::types::Any { unimplemented!() }
    pub fn GetGridHeight(&self, x: i32, y: i32) -> i32 { unimplemented!() }
    pub fn IsBlocked(&self, x: i32, y: i32) -> bool { unimplemented!() }
    pub fn IsNavBlocked(&self) -> bool { unimplemented!() }
    pub fn IsTraversable(&self, x: i32, y: i32) -> bool { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CProjectileManager;

impl CProjectileManager {
    pub fn CreateLinearProjectile(&self, arg1: crate::types::Table) -> i32 { unimplemented!() }
    pub fn CreateTrackingProjectile(&self, arg1: crate::types::Table) -> i32 { unimplemented!() }
    pub fn DestroyLinearProjectile(&self, arg1: i32) { unimplemented!() }
    pub fn GetLinearProjectileLocation(&self, arg1: i32) -> Vector { unimplemented!() }
    pub fn GetLinearProjectileRadius(&self, arg1: i32) -> f64 { unimplemented!() }
    pub fn GetLinearProjectileVelocity(&self, arg1: i32) -> Vector { unimplemented!() }
    pub fn GetTrackingProjectileLocation(&self, arg1: i32) -> Vector { unimplemented!() }
    pub fn GetTrackingProjectileSpeed(&self, arg1: i32) -> f64 { unimplemented!() }
    pub fn ModifyLinearProjectileDamage(&self, arg1: i32, arg2: i32) { unimplemented!() }
    pub fn UpdateLinearProjectileDirection(&self, arg1: i32, arg2: Vector, arg3: f64) { unimplemented!() }
}
