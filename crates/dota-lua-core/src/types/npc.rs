use super::entity::CBaseEntity;
use super::vector::Vector;

#[allow(non_snake_case)]
#[allow(unused)]
pub struct BaseNpc;

#[allow(non_snake_case)]
impl BaseNpc {
    pub fn AddAbility(&self, ability_name: String) -> super::ability::CBaseAbility { unimplemented!() }
    pub fn AddItem(&self, item_to_add: super::item::CBaseItem) -> super::item::CBaseItem { unimplemented!() }
    pub fn AddItemByName(&self, item_name: String) -> super::item::CBaseItem { unimplemented!() }
    pub fn AddNewModifier(&self, caster: Option<BaseNpc>, source_ability: Option<super::ability::CBaseAbility>, modifier_name: String, modifier_table: crate::types::Table) -> super::modifier::CBaseBuff { unimplemented!() }
    pub fn AddNoDraw(&self) { unimplemented!() }
    pub fn AddSpeechBubble(&self, i_bubble: i32, psz_speech: String, fl_duration: f64, un_offset_x: i32, un_offset_y: i32) { unimplemented!() }
    pub fn AlertNearbyUnits(&self, h_attacker: BaseNpc, h_ability: super::ability::CBaseAbility) { unimplemented!() }
    pub fn AngerNearbyUnits(&self) { unimplemented!() }
    pub fn AttackNoEarlierThan(&self, fl_time: f64) { unimplemented!() }
    pub fn AttackReady(&self) -> bool { unimplemented!() }
    pub fn BoundingRadius2D(&self) -> f64 { unimplemented!() }
    pub fn CanEntityBeSeenByMyTeam(&self, h_entity: BaseNpc) -> bool { unimplemented!() }
    pub fn CanSellItems(&self) -> bool { unimplemented!() }
    pub fn CastAbilityImmediately(&self, h_ability: super::ability::CBaseAbility, i_player_index: i32) { unimplemented!() }
    pub fn CastAbilityNoTarget(&self, h_ability: super::ability::CBaseAbility, i_player_index: i32) { unimplemented!() }
    pub fn CastAbilityOnPosition(&self, v_position: Vector, h_ability: super::ability::CBaseAbility, i_player_index: i32) { unimplemented!() }
    pub fn CastAbilityOnTarget(&self, h_target: BaseNpc, h_ability: super::ability::CBaseAbility, i_player_index: i32) { unimplemented!() }
    pub fn CastAbilityToggle(&self, h_ability: super::ability::CBaseAbility, i_player_index: i32) { unimplemented!() }
    pub fn DestroyAllSpeechBubbles(&self) { unimplemented!() }
    pub fn DisassembleItem(&self, h_item: super::item::CBaseItem) { unimplemented!() }
    pub fn DropItemAtPosition(&self, v_dest: Vector, h_item: super::item::CBaseItem) { unimplemented!() }
    pub fn DropItemAtPositionImmediate(&self, h_item: super::item::CBaseItem, v_position: Vector) { unimplemented!() }
    pub fn EjectItemFromStash(&self, h_item: super::item::CBaseItem) { unimplemented!() }
    pub fn FaceTowards(&self, v_target: Vector) { unimplemented!() }
    pub fn FadeGesture(&self, n_activity: i32) { unimplemented!() }
    pub fn FindAbilityByName(&self, ability_name: String) -> super::ability::CBaseAbility { unimplemented!() }
    pub fn FindAllModifiers(&self) -> Vec<super::modifier::CBaseBuff> { unimplemented!() }
    pub fn FindAllModifiersByName(&self, name: String) -> Vec<super::modifier::CBaseBuff> { unimplemented!() }
    pub fn FindItemInInventory(&self, item_name: String) -> super::item::CBaseItem { unimplemented!() }
    pub fn FindModifierByName(&self, modifier_name: String) -> super::modifier::CBaseBuff { unimplemented!() }
    pub fn FindModifierByNameAndCaster(&self, modifier_name: String, caster: BaseNpc) -> super::modifier::CBaseBuff { unimplemented!() }
    pub fn ForceKill(&self, b_reincarnate: bool) { unimplemented!() }
    pub fn ForcePlayActivityOnce(&self, n_activity: i32) { unimplemented!() }
    pub fn GetAbilityByIndex(&self, ability_index: i32) -> super::ability::CBaseAbility { unimplemented!() }
    pub fn GetAbilityCount(&self) -> i32 { unimplemented!() }
    pub fn GetAcquisitionRange(&self) -> i32 { unimplemented!() }
    pub fn GetAdditionalBattleMusicWeight(&self) -> i32 { unimplemented!() }
    pub fn GetAggroTarget(&self) -> BaseNpc { unimplemented!() }
    pub fn GetAttackAnimationPoint(&self) -> f64 { unimplemented!() }
    pub fn GetAttackCapability(&self) -> i32 { unimplemented!() }
    pub fn GetAttackDamage(&self) -> i32 { unimplemented!() }
    pub fn GetAttackRange(&self) -> f64 { unimplemented!() }
    pub fn GetAttackRangeBuffer(&self) -> f64 { unimplemented!() }
    pub fn GetAttackSpeed(&self) -> f64 { unimplemented!() }
    pub fn GetAttackTarget(&self) -> BaseNpc { unimplemented!() }
    pub fn GetAttacksPerSecond(&self) -> f64 { unimplemented!() }
    pub fn GetAverageTrueAttackDamage(&self, h_target: BaseNpc) -> i32 { unimplemented!() }
    pub fn GetBaseAttackTime(&self) -> f64 { unimplemented!() }
    pub fn GetBaseDamageMax(&self) -> i32 { unimplemented!() }
    pub fn GetBaseDamageMin(&self) -> i32 { unimplemented!() }
    pub fn GetBaseDayTimeVisionRange(&self) -> i32 { unimplemented!() }
    pub fn GetBaseHealthRegen(&self) -> f64 { unimplemented!() }
    pub fn GetBaseMagicalResistanceValue(&self) -> f64 { unimplemented!() }
    pub fn GetBaseMaxHealth(&self) -> i32 { unimplemented!() }
    pub fn GetBaseMoveSpeed(&self) -> i32 { unimplemented!() }
    pub fn GetBaseNightTimeVisionRange(&self) -> i32 { unimplemented!() }
    pub fn GetBonusManaRegen(&self) -> f64 { unimplemented!() }
    pub fn GetCastPoint(&self, b_attack: bool) -> f64 { unimplemented!() }
    pub fn GetCloneSource(&self) -> BaseNpc { unimplemented!() }
    pub fn GetCollisionPadding(&self) -> f64 { unimplemented!() }
    pub fn GetCreationTime(&self) -> f64 { unimplemented!() }
    pub fn GetCurrentActiveAbility(&self) -> super::ability::CBaseAbility { unimplemented!() }
    pub fn GetCurrentVisionRange(&self) -> i32 { unimplemented!() }
    pub fn GetCursorCastTarget(&self) -> BaseNpc { unimplemented!() }
    pub fn GetCursorPosition(&self) -> Vector { unimplemented!() }
    pub fn GetCursorTargetingNothing(&self) -> bool { unimplemented!() }
    pub fn GetDayTimeVisionRange(&self) -> i32 { unimplemented!() }
    pub fn GetDeathXP(&self) -> i32 { unimplemented!() }
    pub fn GetForceAttackTarget(&self) -> BaseNpc { unimplemented!() }
    pub fn GetGoldBounty(&self) -> i32 { unimplemented!() }
    pub fn GetHasteFactor(&self) -> f64 { unimplemented!() }
    pub fn GetHealthDeficit(&self) -> i32 { unimplemented!() }
    pub fn GetHealthPercent(&self) -> f64 { unimplemented!() }
    pub fn GetHealthRegen(&self) -> f64 { unimplemented!() }
    pub fn GetHullRadius(&self) -> f64 { unimplemented!() }
    pub fn GetIdealSpeed(&self) -> f64 { unimplemented!() }
    pub fn GetIdealSpeedNoSlows(&self) -> f64 { unimplemented!() }
    pub fn GetIncreasedAttackSpeed(&self) -> f64 { unimplemented!() }
    pub fn GetInitialGoalEntity(&self) -> CBaseEntity { unimplemented!() }
    pub fn GetItemInSlot(&self, slot: i32) -> super::item::CBaseItem { unimplemented!() }
    pub fn GetLastAttackTime(&self) -> f64 { unimplemented!() }
    pub fn GetLastIdleChangeTime(&self) -> f64 { unimplemented!() }
    pub fn GetLevel(&self) -> i32 { unimplemented!() }
    pub fn GetMagicalArmorValue(&self) -> f64 { unimplemented!() }
    pub fn GetMainControllingPlayer(&self) -> i32 { unimplemented!() }
    pub fn GetMana(&self) -> f64 { unimplemented!() }
    pub fn GetManaPercent(&self) -> f64 { unimplemented!() }
    pub fn GetManaRegen(&self) -> f64 { unimplemented!() }
    pub fn GetManaRegenMultiplier(&self) -> f64 { unimplemented!() }
    pub fn GetMaxMana(&self) -> f64 { unimplemented!() }
    pub fn GetMaximumGoldBounty(&self) -> i32 { unimplemented!() }
    pub fn GetMinimumGoldBounty(&self) -> i32 { unimplemented!() }
    pub fn GetModelRadius(&self) -> f64 { unimplemented!() }
    pub fn GetModifierCount(&self) -> i32 { unimplemented!() }
    pub fn GetModifierNameByIndex(&self, n_index: i32) -> String { unimplemented!() }
    pub fn GetModifierStackCount(&self, modifier_name: String, h_caster: BaseNpc) -> i32 { unimplemented!() }
    pub fn GetMoveSpeedModifier(&self, fl_base_speed: f64) -> f64 { unimplemented!() }
    pub fn GetMustReachEachGoalEntity(&self) -> bool { unimplemented!() }
    pub fn GetNeverMoveToClearSpace(&self) -> bool { unimplemented!() }
    pub fn GetNightTimeVisionRange(&self) -> i32 { unimplemented!() }
    pub fn GetOpposingTeamNumber(&self) -> i32 { unimplemented!() }
    pub fn GetPaddedCollisionRadius(&self) -> f64 { unimplemented!() }
    pub fn GetPhysicalArmorBaseValue(&self) -> f64 { unimplemented!() }
    pub fn GetPhysicalArmorValue(&self) -> f64 { unimplemented!() }
    pub fn GetPlayerOwner(&self) -> super::npc::CDOTAPlayer { unimplemented!() }
    pub fn GetPlayerOwnerID(&self) -> i32 { unimplemented!() }
    pub fn GetProjectileSpeed(&self) -> i32 { unimplemented!() }
    pub fn GetRangeToUnit(&self, h_npc: BaseNpc) -> f64 { unimplemented!() }
    pub fn GetRangedProjectileName(&self) -> String { unimplemented!() }
    pub fn GetSecondsPerAttack(&self) -> f64 { unimplemented!() }
    pub fn GetTotalPurchasedUpgradeGoldCost(&self) -> i32 { unimplemented!() }
    pub fn GetUnitLabel(&self) -> String { unimplemented!() }
    pub fn GetUnitName(&self) -> String { unimplemented!() }
    pub fn GiveMana(&self, fl_mana: f64) { unimplemented!() }
    pub fn HasAbility(&self, psz_ability_name: String) -> bool { unimplemented!() }
    pub fn HasAnyActiveAbilities(&self) -> bool { unimplemented!() }
    pub fn HasAttackCapability(&self) -> bool { unimplemented!() }
    pub fn HasFlyMovementCapability(&self) -> bool { unimplemented!() }
    pub fn HasFlyingVision(&self) -> bool { unimplemented!() }
    pub fn HasGroundMovementCapability(&self) -> bool { unimplemented!() }
    pub fn HasInventory(&self) -> bool { unimplemented!() }
    pub fn HasItemInInventory(&self, p_item_name: String) -> bool { unimplemented!() }
    pub fn HasModifier(&self, psz_script_name: String) -> bool { unimplemented!() }
    pub fn HasMovementCapability(&self) -> bool { unimplemented!() }
    pub fn HasScepter(&self) -> bool { unimplemented!() }
    pub fn Heal(&self, fl_amount: f64, h_inflictor: BaseNpc) { unimplemented!() }
    pub fn Hold(&self) { unimplemented!() }
    pub fn Interrupt(&self) { unimplemented!() }
    pub fn InterruptChannel(&self) { unimplemented!() }
    pub fn InterruptMotionControllers(&self, b_find_clear_space: bool) { unimplemented!() }
    pub fn IsAlive(&self) -> bool { unimplemented!() }
    pub fn IsAncient(&self) -> bool { unimplemented!() }
    pub fn IsAttackImmune(&self) -> bool { unimplemented!() }
    pub fn IsAttacking(&self) -> bool { unimplemented!() }
    pub fn IsAttackingEntity(&self, h_entity: BaseNpc) -> bool { unimplemented!() }
    pub fn IsBarracks(&self) -> bool { unimplemented!() }
    pub fn IsBlind(&self) -> bool { unimplemented!() }
    pub fn IsBlockDisabled(&self) -> bool { unimplemented!() }
    pub fn IsBoss(&self) -> bool { unimplemented!() }
    pub fn IsBuilding(&self) -> bool { unimplemented!() }
    pub fn IsChanneling(&self) -> bool { unimplemented!() }
    pub fn IsClone(&self) -> bool { unimplemented!() }
    pub fn IsCommandRestricted(&self) -> bool { unimplemented!() }
    pub fn IsConsideredHero(&self) -> bool { unimplemented!() }
    pub fn IsControllableByAnyPlayer(&self) -> bool { unimplemented!() }
    pub fn IsCourier(&self) -> bool { unimplemented!() }
    pub fn IsCreature(&self) -> bool { unimplemented!() }
    pub fn IsCreep(&self) -> bool { unimplemented!() }
    pub fn IsDeniable(&self) -> bool { unimplemented!() }
    pub fn IsDisarmed(&self) -> bool { unimplemented!() }
    pub fn IsDominated(&self) -> bool { unimplemented!() }
    pub fn IsEvadeDisabled(&self) -> bool { unimplemented!() }
    pub fn IsFort(&self) -> bool { unimplemented!() }
    pub fn IsFrozen(&self) -> bool { unimplemented!() }
    pub fn IsHero(&self) -> bool { unimplemented!() }
    pub fn IsHexed(&self) -> bool { unimplemented!() }
    pub fn IsIdle(&self) -> bool { unimplemented!() }
    pub fn IsIllusion(&self) -> bool { unimplemented!() }
    pub fn IsInvisible(&self) -> bool { unimplemented!() }
    pub fn IsInvulnerable(&self) -> bool { unimplemented!() }
    pub fn IsLowAttackPriority(&self) -> bool { unimplemented!() }
    pub fn IsMagicImmune(&self) -> bool { unimplemented!() }
    pub fn IsMovementImpaired(&self) -> bool { unimplemented!() }
    pub fn IsMoving(&self) -> bool { unimplemented!() }
    pub fn IsMuted(&self) -> bool { unimplemented!() }
    pub fn IsNeutralUnitType(&self) -> bool { unimplemented!() }
    pub fn IsNightmared(&self) -> bool { unimplemented!() }
    pub fn IsOpposingTeam(&self, n_team: i32) -> bool { unimplemented!() }
    pub fn IsOther(&self) -> bool { unimplemented!() }
    pub fn IsOutOfGame(&self) -> bool { unimplemented!() }
    pub fn IsOwnedByAnyPlayer(&self) -> bool { unimplemented!() }
    pub fn IsPhantom(&self) -> bool { unimplemented!() }
    pub fn IsPhantomBlocker(&self) -> bool { unimplemented!() }
    pub fn IsPhased(&self) -> bool { unimplemented!() }
    pub fn IsPositionInRange(&self, v_position: Vector, fl_range: f64) -> bool { unimplemented!() }
    pub fn IsRangedAttacker(&self) -> bool { unimplemented!() }
    pub fn IsRealHero(&self) -> bool { unimplemented!() }
    pub fn IsRooted(&self) -> bool { unimplemented!() }
    pub fn IsShrine(&self) -> bool { unimplemented!() }
    pub fn IsSilenced(&self) -> bool { unimplemented!() }
    pub fn IsSpeciallyDeniable(&self) -> bool { unimplemented!() }
    pub fn IsStunned(&self) -> bool { unimplemented!() }
    pub fn IsSummoned(&self) -> bool { unimplemented!() }
    pub fn IsTempestDouble(&self) -> bool { unimplemented!() }
    pub fn IsTower(&self) -> bool { unimplemented!() }
    pub fn IsUnableToMiss(&self) -> bool { unimplemented!() }
    pub fn IsUnselectable(&self) -> bool { unimplemented!() }
    pub fn IsUntargetable(&self) -> bool { unimplemented!() }
    pub fn Kill(&self, h_ability: Option<super::ability::CBaseAbility>, h_attacker: Option<BaseNpc>) { unimplemented!() }
    pub fn MakeIllusion(&self) { unimplemented!() }
    pub fn MakePhantomBlocker(&self) { unimplemented!() }
    pub fn MakeVisibleDueToAttack(&self, i_team: i32) { unimplemented!() }
    pub fn MakeVisibleToTeam(&self, i_team: i32, fl_duration: f64) { unimplemented!() }
    pub fn ManageModelChanges(&self) { unimplemented!() }
    pub fn ModifyHealth(&self, i_desired_health_value: i32, h_ability: super::ability::CBaseAbility, b_lethal: bool, i_additional_flags: i32) { unimplemented!() }
    pub fn MoveToNPC(&self, h_npc: BaseNpc) { unimplemented!() }
    pub fn MoveToNPCToGiveItem(&self, h_npc: BaseNpc, h_item: super::item::CBaseItem) { unimplemented!() }
    pub fn MoveToPosition(&self, v_dest: Vector) { unimplemented!() }
    pub fn MoveToPositionAggressive(&self, v_dest: Vector) { unimplemented!() }
    pub fn MoveToTargetToAttack(&self, h_target: BaseNpc) { unimplemented!() }
    pub fn NoHealthBar(&self) -> bool { unimplemented!() }
    pub fn NoTeamMoveTo(&self) -> bool { unimplemented!() }
    pub fn NoTeamSelect(&self) -> bool { unimplemented!() }
    pub fn NoUnitCollision(&self) -> bool { unimplemented!() }
    pub fn NotOnMinimap(&self) -> bool { unimplemented!() }
    pub fn NotOnMinimapForEnemies(&self) -> bool { unimplemented!() }
    pub fn NotifyWearablesOfModelChange(&self, b_original_model: bool) { unimplemented!() }
    pub fn PassivesDisabled(&self) -> bool { unimplemented!() }
    pub fn PatrolToPosition(&self, v_dest: Vector) { unimplemented!() }
    pub fn PerformAttack(&self, h_target: BaseNpc, b_use_cast_attack_orb: bool, b_process_procs: bool, b_skip_cooldown: bool, b_ignore_invis: bool, b_use_projectile: bool, b_fake_attack: bool, b_never_miss: bool) { unimplemented!() }
    pub fn PickupDroppedItem(&self, h_item: super::item::CBaseItem) { unimplemented!() }
    pub fn PickupRune(&self, h_item: super::item::CBaseItem) { unimplemented!() }
    pub fn PlayVCD(&self, p_vcd: String) { unimplemented!() }
    pub fn ProvidesVision(&self) -> bool { unimplemented!() }
    pub fn Purge(&self, b_remove_positive_buffs: bool, b_remove_debuffs: bool, b_frame_only: bool, b_remove_stuns: bool, b_remove_exceptions: bool) { unimplemented!() }
    pub fn ReduceMana(&self, fl_amount: f64) { unimplemented!() }
    pub fn RemoveAbility(&self, psz_ability_name: String) { unimplemented!() }
    pub fn RemoveGesture(&self, n_activity: i32) { unimplemented!() }
    pub fn RemoveHorizontalMotionController(&self, h_buff: super::modifier::CBaseBuff) { unimplemented!() }
    pub fn RemoveItem(&self, h_item: super::item::CBaseItem) { unimplemented!() }
    pub fn RemoveModifierByName(&self, psz_script_name: String) { unimplemented!() }
    pub fn RemoveModifierByNameAndCaster(&self, psz_script_name: String, h_caster: BaseNpc) { unimplemented!() }
    pub fn RemoveNoDraw(&self) { unimplemented!() }
    pub fn RemoveVerticalMotionController(&self, h_buff: super::modifier::CBaseBuff) { unimplemented!() }
    pub fn RespawnUnit(&self) { unimplemented!() }
    pub fn SellItem(&self, h_item: super::item::CBaseItem) { unimplemented!() }
    pub fn SetAbilityByIndex(&self, h_ability: super::ability::CBaseAbility, i_index: i32) { unimplemented!() }
    pub fn SetAcquisitionRange(&self, n_range: i32) { unimplemented!() }
    pub fn SetAdditionalBattleMusicWeight(&self, fl_weight: f64) { unimplemented!() }
    pub fn SetAggroTarget(&self, h_aggro_target: BaseNpc) { unimplemented!() }
    pub fn SetAttackCapability(&self, i_attack_capabilities: i32) { unimplemented!() }
    pub fn SetAttacking(&self, h_attack_target: BaseNpc) { unimplemented!() }
    pub fn SetBaseAttackTime(&self, fl_base_attack_time: f64) { unimplemented!() }
    pub fn SetBaseDamageMax(&self, n_max: i32) { unimplemented!() }
    pub fn SetBaseDamageMin(&self, n_min: i32) { unimplemented!() }
    pub fn SetBaseHealthRegen(&self, fl_health_regen: f64) { unimplemented!() }
    pub fn SetBaseMagicalResistanceValue(&self, fl_magical_resistance_value: f64) { unimplemented!() }
    pub fn SetBaseManaRegen(&self, fl_mana_regen: f64) { unimplemented!() }
    pub fn SetBaseMaxHealth(&self, fl_base_max_health: f64) { unimplemented!() }
    pub fn SetBaseMoveSpeed(&self, i_move_speed: i32) { unimplemented!() }
    pub fn SetCanSellItems(&self, b_can_sell: bool) { unimplemented!() }
    pub fn SetControllableByPlayer(&self, i_index: i32, b_skip_adjusting_position: bool) { unimplemented!() }
    pub fn SetCursorCastTarget(&self, h_entity: BaseNpc) { unimplemented!() }
    pub fn SetCursorPosition(&self, v_location: Vector) { unimplemented!() }
    pub fn SetCursorTargetingNothing(&self, b_targeting_nothing: bool) { unimplemented!() }
    pub fn SetCustomHealthLabel(&self, p_label: String, r: i32, g: i32, b: i32) { unimplemented!() }
    pub fn SetDayTimeVisionRange(&self, i_range: i32) { unimplemented!() }
    pub fn SetDeathXP(&self, i_xp_bounty: i32) { unimplemented!() }
    pub fn SetForceAttackTarget(&self, h_npc: BaseNpc) { unimplemented!() }
    pub fn SetForceAttackTargetAlly(&self, h_npc: BaseNpc) { unimplemented!() }
    pub fn SetHasInventory(&self, b_has_inventory: bool) { unimplemented!() }
    pub fn SetHullRadius(&self, fl_hull_radius: f64) { unimplemented!() }
    pub fn SetIdleAcquire(&self, b_idle_acquire: bool) { unimplemented!() }
    pub fn SetInitialGoalEntity(&self, h_goal: CBaseEntity) { unimplemented!() }
    pub fn SetMana(&self, fl_mana: f64) { unimplemented!() }
    pub fn SetMaximumGoldBounty(&self, i_gold_bounty_max: i32) { unimplemented!() }
    pub fn SetMinimumGoldBounty(&self, i_gold_bounty_min: i32) { unimplemented!() }
    pub fn SetModifierStackCount(&self, psz_script_name: String, h_caster: BaseNpc, n_stack_count: i32) { unimplemented!() }
    pub fn SetMoveCapability(&self, i_move_capabilities: i32) { unimplemented!() }
    pub fn SetMustReachEachGoalEntity(&self, must: bool) { unimplemented!() }
    pub fn SetNeverMoveToClearSpace(&self, never_move_to_clear_space: bool) { unimplemented!() }
    pub fn SetNightTimeVisionRange(&self, i_range: i32) { unimplemented!() }
    pub fn SetOrigin(&self, v_location: Vector) { unimplemented!() }
    pub fn SetOriginalModel(&self, psz_model_name: String) { unimplemented!() }
    pub fn SetPhysicalArmorBaseValue(&self, fl_physical_armor_value: f64) { unimplemented!() }
    pub fn SetRangedProjectileName(&self, p_projectile_name: String) { unimplemented!() }
    pub fn SetRevealRadius(&self, reveal_radius: i32) { unimplemented!() }
    pub fn SetStolenScepter(&self, b_stolen_scepter: bool) { unimplemented!() }
    pub fn SetUnitCanRespawn(&self, b_can_respawn: bool) { unimplemented!() }
    pub fn SetUnitName(&self, p_name: String) { unimplemented!() }
    pub fn ShouldIdleAcquire(&self) -> bool { unimplemented!() }
    pub fn SpendMana(&self, fl_mana_spent: f64, h_ability: super::ability::CBaseAbility) { unimplemented!() }
    pub fn StartGesture(&self, n_activity: i32) { unimplemented!() }
    pub fn StartGestureWithPlaybackRate(&self, n_activity: i32, fl_rate: f64) { unimplemented!() }
    pub fn Stop(&self) { unimplemented!() }
    pub fn StopFacing(&self) { unimplemented!() }
    pub fn SwapAbilities(&self, p_ability_name1: String, p_ability_name2: String, b_enable1: bool, b_enable2: bool) { unimplemented!() }
    pub fn SwapItems(&self, n_slot1: i32, n_slot2: i32) { unimplemented!() }
    pub fn TakeItem(&self, h_item: super::item::CBaseItem) -> super::item::CBaseItem { unimplemented!() }
    pub fn TimeUntilNextAttack(&self) -> f64 { unimplemented!() }
    pub fn TriggerModifierDodge(&self) -> bool { unimplemented!() }
    pub fn TriggerSpellAbsorb(&self, h_ability: super::ability::CBaseAbility) -> bool { unimplemented!() }
    pub fn TriggerSpellReflect(&self, h_ability: super::ability::CBaseAbility) { unimplemented!() }
    pub fn UnHideAbilityToSlot(&self, psz_ability_name: String, psz_replaced_ability_name: String) { unimplemented!() }
    pub fn UnitCanRespawn(&self) -> bool { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct BaseNpcBuilding;

impl BaseNpcBuilding {
    pub fn GetInvulnCount(&self) -> i32 { unimplemented!() }
    pub fn SetInvulnCount(&self, n_invuln_count: i32) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct BaseNpcCreature;

impl BaseNpcCreature {
    pub fn AddItemDrop(&self, h_drop_data: crate::types::Table) { unimplemented!() }
    pub fn CreatureLevelUp(&self, i_levels: i32) { unimplemented!() }
    pub fn IsChampion(&self) -> bool { unimplemented!() }
    pub fn RemoveAllItemDrops(&self) { unimplemented!() }
    pub fn SetArmorGain(&self, fl_armor_gain: f64) { unimplemented!() }
    pub fn SetAttackTimeGain(&self, fl_attack_time_gain: f64) { unimplemented!() }
    pub fn SetBountyGain(&self, n_bounty_gain: i32) { unimplemented!() }
    pub fn SetChampion(&self, b_is_champion: bool) { unimplemented!() }
    pub fn SetDamageGain(&self, n_damage_gain: i32) { unimplemented!() }
    pub fn SetDisableResistanceGain(&self, fl_disable_resistance_gain: f64) { unimplemented!() }
    pub fn SetHPGain(&self, n_hp_gain: i32) { unimplemented!() }
    pub fn SetHPRegenGain(&self, fl_hp_regen_gain: f64) { unimplemented!() }
    pub fn SetMagicResistanceGain(&self, fl_magic_resistance_gain: f64) { unimplemented!() }
    pub fn SetManaGain(&self, n_mana_gain: i32) { unimplemented!() }
    pub fn SetManaRegenGain(&self, fl_mana_regen_gain: f64) { unimplemented!() }
    pub fn SetMoveSpeedGain(&self, n_move_speed_gain: i32) { unimplemented!() }
    pub fn SetXPGain(&self, n_xp_gain: i32) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct BaseNpcHero;

impl BaseNpcHero {
    pub fn AddExperience(&self, fl_xp: f64, n_reason: i32, b_apply_bot_difficulty_scaling: bool, b_increment_total: bool) -> bool { unimplemented!() }
    pub fn Buyback(&self) { unimplemented!() }
    pub fn CalculateStatBonus(&self) { unimplemented!() }
    pub fn CanEarnGold(&self) -> bool { unimplemented!() }
    pub fn ClearLastHitMultikill(&self) { unimplemented!() }
    pub fn ClearLastHitStreak(&self) { unimplemented!() }
    pub fn ClearStreak(&self) { unimplemented!() }
    pub fn GetAbilityPoints(&self) -> i32 { unimplemented!() }
    pub fn GetAdditionalOwnedUnits(&self) -> Vec<BaseNpc> { unimplemented!() }
    pub fn GetAgility(&self) -> f64 { unimplemented!() }
    pub fn GetAgilityGain(&self) -> f64 { unimplemented!() }
    pub fn GetAssists(&self) -> i32 { unimplemented!() }
    pub fn GetAttacker(&self, n_index: i32) -> i32 { unimplemented!() }
    pub fn GetBaseAgility(&self) -> f64 { unimplemented!() }
    pub fn GetBaseIntellect(&self) -> f64 { unimplemented!() }
    pub fn GetBaseManaRegen(&self) -> f64 { unimplemented!() }
    pub fn GetBaseStrength(&self) -> f64 { unimplemented!() }
    pub fn GetBonusDamageFromPrimaryStat(&self) -> i32 { unimplemented!() }
    pub fn GetBuybackCooldownTime(&self) -> f64 { unimplemented!() }
    pub fn GetBuybackCost(&self) -> i32 { unimplemented!() }
    pub fn GetBuybackGoldLimitTime(&self) -> f64 { unimplemented!() }
    pub fn GetCurrentXP(&self) -> f64 { unimplemented!() }
    pub fn GetDeathGoldCost(&self) -> i32 { unimplemented!() }
    pub fn GetDeaths(&self) -> i32 { unimplemented!() }
    pub fn GetDenies(&self) -> i32 { unimplemented!() }
    pub fn GetGold(&self) -> i32 { unimplemented!() }
    pub fn GetGoldBounty(&self) -> i32 { unimplemented!() }
    pub fn GetIntellect(&self) -> f64 { unimplemented!() }
    pub fn GetIntellectGain(&self) -> f64 { unimplemented!() }
    pub fn GetKills(&self) -> i32 { unimplemented!() }
    pub fn GetLastHits(&self) -> i32 { unimplemented!() }
    pub fn GetManaRegenMultiplier(&self) -> f64 { unimplemented!() }
    pub fn GetMostRecentDamageTime(&self) -> f64 { unimplemented!() }
    pub fn GetMultipleKillCount(&self) -> i32 { unimplemented!() }
    pub fn GetNumAttackers(&self) -> i32 { unimplemented!() }
    pub fn GetNumItemsInInventory(&self) -> i32 { unimplemented!() }
    pub fn GetNumItemsInStash(&self) -> i32 { unimplemented!() }
    pub fn GetPhysicalArmorBaseValue(&self) -> f64 { unimplemented!() }
    pub fn GetPlayerID(&self) -> i32 { unimplemented!() }
    pub fn GetPrimaryAttribute(&self) -> i32 { unimplemented!() }
    pub fn GetPrimaryStatValue(&self) -> f64 { unimplemented!() }
    pub fn GetRespawnTime(&self) -> f64 { unimplemented!() }
    pub fn GetRespawnsDisabled(&self) -> bool { unimplemented!() }
    pub fn GetStreak(&self) -> i32 { unimplemented!() }
    pub fn GetStrength(&self) -> f64 { unimplemented!() }
    pub fn GetStrengthGain(&self) -> f64 { unimplemented!() }
    pub fn GetTimeUntilRespawn(&self) -> f64 { unimplemented!() }
    pub fn GetTogglableWearable(&self, n_slot_type: i32) -> CBaseEntity { unimplemented!() }
    pub fn HasAnyAvailableInventorySpace(&self) -> bool { unimplemented!() }
    pub fn HasFlyingVision(&self) -> bool { unimplemented!() }
    pub fn HasOwnerAbandoned(&self) -> bool { unimplemented!() }
    pub fn HasRoomForItem(&self, p_item_name: String, b_include_stash_combines: bool, b_allow_selling: bool) -> i32 { unimplemented!() }
    pub fn HeroLevelUp(&self, b_play_effects: bool) { unimplemented!() }
    pub fn IncrementAssists(&self, i_killer_id: i32) { unimplemented!() }
    pub fn IncrementDeaths(&self, i_killer_id: i32) { unimplemented!() }
    pub fn IncrementDenies(&self) { unimplemented!() }
    pub fn IncrementKills(&self, i_victim_id: i32) { unimplemented!() }
    pub fn IncrementLastHitMultikill(&self) { unimplemented!() }
    pub fn IncrementLastHitStreak(&self) { unimplemented!() }
    pub fn IncrementLastHits(&self) { unimplemented!() }
    pub fn IncrementNearbyCreepDeaths(&self) { unimplemented!() }
    pub fn IncrementStreak(&self) { unimplemented!() }
    pub fn IsBuybackDisabledByReapersScythe(&self) -> bool { unimplemented!() }
    pub fn IsReincarnating(&self) -> bool { unimplemented!() }
    pub fn IsStashEnabled(&self) -> bool { unimplemented!() }
    pub fn KilledHero(&self, h_hero: BaseNpcHero, h_inflictor: BaseNpc) { unimplemented!() }
    pub fn ModifyAgility(&self, fl_new_agility: f64) { unimplemented!() }
    pub fn ModifyGold(&self, i_gold_change: i32, b_reliable: bool, i_reason: i32) -> i32 { unimplemented!() }
    pub fn ModifyIntellect(&self, fl_new_intellect: f64) { unimplemented!() }
    pub fn ModifyStrength(&self, fl_new_strength: f64) { unimplemented!() }
    pub fn PerformTaunt(&self) { unimplemented!() }
    pub fn RecordLastHit(&self) { unimplemented!() }
    pub fn RespawnHero(&self, b_buy_back: bool, b_respawn_penalty: bool) { unimplemented!() }
    pub fn SetAbilityPoints(&self, i_points: i32) { unimplemented!() }
    pub fn SetBaseAgility(&self, fl_agility: f64) { unimplemented!() }
    pub fn SetBaseIntellect(&self, fl_intellect: f64) { unimplemented!() }
    pub fn SetBaseStrength(&self, fl_strength: f64) { unimplemented!() }
    pub fn SetBotDifficulty(&self, n_difficulty: i32) { unimplemented!() }
    pub fn SetBuyBackDisabledByReapersScythe(&self, b_buyback_disabled: bool) { unimplemented!() }
    pub fn SetBuybackCooldownTime(&self, fl_time: f64) { unimplemented!() }
    pub fn SetBuybackGoldLimitTime(&self, fl_time: f64) { unimplemented!() }
    pub fn SetCustomDeathXP(&self, i_value: i32) { unimplemented!() }
    pub fn SetGold(&self, i_gold: i32, b_reliable: bool) { unimplemented!() }
    pub fn SetPlayerID(&self, i_player_id: i32) { unimplemented!() }
    pub fn SetPrimaryAttribute(&self, n_primary_attribute: i32) { unimplemented!() }
    pub fn SetRespawnPosition(&self, v_origin: Vector) { unimplemented!() }
    pub fn SetRespawnsDisabled(&self, b_disable_respawns: bool) { unimplemented!() }
    pub fn SetStashEnabled(&self, b_enabled: bool) { unimplemented!() }
    pub fn SetTimeUntilRespawn(&self, time: f64) { unimplemented!() }
    pub fn ShouldDoFlyHeightVisual(&self) -> bool { unimplemented!() }
    pub fn SpendGold(&self, i_cost: i32, i_reason: i32) { unimplemented!() }
    pub fn UnitCanRespawn(&self) -> bool { unimplemented!() }
    pub fn UpgradeAbility(&self, h_ability: super::ability::CBaseAbility) { unimplemented!() }
    pub fn WillReincarnate(&self) -> bool { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct BaseNpcTrapWard;

impl BaseNpcTrapWard {
    pub fn GetTrapTarget(&self) -> Vector { unimplemented!() }
    pub fn SetAnimation(&self, p_animation: String) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CDOTAPlayer;

impl CDOTAPlayer {
    pub fn GetAssignedHero(&self) -> BaseNpcHero { unimplemented!() }
    pub fn GetPlayerID(&self) -> i32 { unimplemented!() }
    pub fn MakeRandomHeroSelection(&self) { unimplemented!() }
    pub fn SetKillCamUnit(&self, h_entity: BaseNpc) { unimplemented!() }
    pub fn SetMusicStatus(&self, n_music_status: i32, fl_intensity: f64) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CDOTAMapTree;

impl CDOTAMapTree {
    pub fn CutDown(&self, n_tree_number_known_to: i32) { unimplemented!() }
    pub fn CutDownRegrowAfter(&self, fl_regrow_after: f64, n_tree_number_known_to: i32) { unimplemented!() }
    pub fn GrowBack(&self) { unimplemented!() }
    pub fn IsStanding(&self) -> bool { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CDOTASimpleObstruction;

impl CDOTASimpleObstruction {
    pub fn IsEnabled(&self) -> bool { unimplemented!() }
    pub fn SetEnabled(&self, b_enabled: bool, b_force: bool) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CDOTAUnitCourier;

impl CDOTAUnitCourier {
    pub fn UpgradeToFlyingCourier(&self) -> bool { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CDotaQuest;

impl CDotaQuest {
    pub fn AddSubquest(&self, h_subquest: crate::types::Table) { unimplemented!() }
    pub fn CompleteQuest(&self) { unimplemented!() }
    pub fn GetSubquest(&self, n_index: i32) -> CDotaSubquestBase { unimplemented!() }
    pub fn GetSubquestByName(&self, psz_name: String) -> CDotaSubquestBase { unimplemented!() }
    pub fn RemoveSubquest(&self, h_subquest: CDotaSubquestBase) { unimplemented!() }
    pub fn SetTextReplaceString(&self, psz_string: String) { unimplemented!() }
    pub fn SetTextReplaceValue(&self, value_slot: i32, value: i32) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CDotaSubquestBase;

impl CDotaSubquestBase {
    pub fn CompleteSubquest(&self) { unimplemented!() }
    pub fn SetTextReplaceString(&self, psz_string: String) { unimplemented!() }
    pub fn SetTextReplaceValue(&self, value_slot: i32, value: i32) { unimplemented!() }
}
