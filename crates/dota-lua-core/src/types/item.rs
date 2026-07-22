use super::entity::CBaseEntity;
use super::npc::BaseNpc;
use super::vector::Vector;

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseItem;

impl CBaseItem {
    pub fn CanBeUsedOutOfInventory(&self) -> bool { unimplemented!() }
    pub fn GetContainer(&self) -> super::misc::CBaseItemPhysical { unimplemented!() }
    pub fn GetCost(&self) -> i32 { unimplemented!() }
    pub fn GetCurrentCharges(&self) -> i32 { unimplemented!() }
    pub fn GetInitialCharges(&self) -> i32 { unimplemented!() }
    pub fn GetItemState(&self) -> i32 { unimplemented!() }
    pub fn GetPurchaseTime(&self) -> f64 { unimplemented!() }
    pub fn GetPurchaser(&self) -> BaseNpc { unimplemented!() }
    pub fn GetShareability(&self) -> i32 { unimplemented!() }
    pub fn IsAlertableItem(&self) -> bool { unimplemented!() }
    pub fn IsCastOnPickup(&self) -> bool { unimplemented!() }
    pub fn IsCombinable(&self) -> bool { unimplemented!() }
    pub fn IsDisassemblable(&self) -> bool { unimplemented!() }
    pub fn IsDroppable(&self) -> bool { unimplemented!() }
    pub fn IsInBackpack(&self) -> bool { unimplemented!() }
    pub fn IsKillable(&self) -> bool { unimplemented!() }
    pub fn IsMuted(&self) -> bool { unimplemented!() }
    pub fn IsPermanent(&self) -> bool { unimplemented!() }
    pub fn IsPurchasable(&self) -> bool { unimplemented!() }
    pub fn IsRecipe(&self) -> bool { unimplemented!() }
    pub fn IsRecipeGenerated(&self) -> bool { unimplemented!() }
    pub fn IsSellable(&self) -> bool { unimplemented!() }
    pub fn IsStackable(&self) -> bool { unimplemented!() }
    pub fn LaunchLoot(&self, b_auto_use: bool, fl_height: f64, fl_duration: f64, v_end_point: Vector) { unimplemented!() }
    pub fn LaunchLootInitialHeight(&self, b_auto_use: bool, fl_initial_height: f64, fl_launch_height: f64, fl_duration: f64, v_end_point: Vector) { unimplemented!() }
    pub fn OnEquip(&self) { unimplemented!() }
    pub fn OnUnequip(&self) { unimplemented!() }
    pub fn RequiresCharges(&self) -> bool { unimplemented!() }
    pub fn SetCanBeUsedOutOfInventory(&self, b_value: bool) { unimplemented!() }
    pub fn SetCastOnPickup(&self, b_cast_on_pick_up: bool) { unimplemented!() }
    pub fn SetCurrentCharges(&self, i_charges: i32) { unimplemented!() }
    pub fn SetDroppable(&self, b_droppable: bool) { unimplemented!() }
    pub fn SetItemState(&self, i_state: i32) { unimplemented!() }
    pub fn SetPurchaseTime(&self, fl_time: f64) { unimplemented!() }
    pub fn SetPurchaser(&self, h_purchaser: BaseNpc) { unimplemented!() }
    pub fn SetSellable(&self, b_sellable: bool) { unimplemented!() }
    pub fn SetShareability(&self, i_shareability: i32) { unimplemented!() }
    pub fn SetStacksWithOtherOwners(&self, b_stacks_with_other_owners: bool) { unimplemented!() }
    pub fn SpendCharge(&self) { unimplemented!() }
    pub fn StacksWithOtherOwners(&self) -> bool { unimplemented!() }
    pub fn Think(&self) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseItemSpawner;

impl CBaseItemSpawner {
    pub fn GetItemName(&self) -> String { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseItemDataDriven;

impl CBaseItemDataDriven {
    pub fn ApplyDataDrivenModifier(&self, h_caster: BaseNpc, h_target: BaseNpc, psz_modifier_name: String, h_modifier_table: crate::types::Table) { unimplemented!() }
    pub fn ApplyDataDrivenThinker(&self, h_caster: BaseNpc, v_location: Vector, psz_modifier_name: String, h_modifier_table: crate::types::Table) -> CBaseEntity { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseItemLua;

impl CBaseItemLua {
    pub fn CastFilterResult(&self) -> i32 { unimplemented!() }
    pub fn CastFilterResultLocation(&self, v_location: Vector) -> i32 { unimplemented!() }
    pub fn CastFilterResultTarget(&self, h_target: BaseNpc) -> i32 { unimplemented!() }
    pub fn GetAssociatedPrimaryAbilities(&self) -> String { unimplemented!() }
    pub fn GetAssociatedSecondaryAbilities(&self) -> String { unimplemented!() }
    pub fn GetBehavior(&self) -> i64 { unimplemented!() }
    pub fn GetCastRange(&self, v_location: Vector, h_target: BaseNpc) -> i32 { unimplemented!() }
    pub fn GetChannelTime(&self) -> f64 { unimplemented!() }
    pub fn GetChannelledManaCostPerSecond(&self, i_level: i32) -> i32 { unimplemented!() }
    pub fn GetConceptRecipientType(&self) -> i32 { unimplemented!() }
    pub fn GetCooldown(&self, i_level: i32) -> f64 { unimplemented!() }
    pub fn GetCustomCastError(&self) -> String { unimplemented!() }
    pub fn GetCustomCastErrorLocation(&self, v_location: Vector) -> String { unimplemented!() }
    pub fn GetCustomCastErrorTarget(&self, h_target: BaseNpc) -> String { unimplemented!() }
    pub fn GetGoldCost(&self, i_level: i32) -> i32 { unimplemented!() }
    pub fn GetIntrinsicModifierName(&self) -> String { unimplemented!() }
    pub fn GetManaCost(&self, i_level: i32) -> i32 { unimplemented!() }
    pub fn GetPlaybackRateOverride(&self) -> f64 { unimplemented!() }
    pub fn IsHiddenAbilityCastable(&self) -> bool { unimplemented!() }
    pub fn IsHiddenWhenStolen(&self) -> bool { unimplemented!() }
    pub fn IsRefreshable(&self) -> bool { unimplemented!() }
    pub fn IsStealable(&self) -> bool { unimplemented!() }
    pub fn OnAbilityPhaseInterrupted(&self) { unimplemented!() }
    pub fn OnAbilityPhaseStart(&self) -> bool { unimplemented!() }
    pub fn OnChannelFinish(&self, b_interrupted: bool) { unimplemented!() }
    pub fn OnChannelThink(&self, fl_interval: f64) { unimplemented!() }
    pub fn OnHeroCalculateStatBonus(&self) { unimplemented!() }
    pub fn OnHeroDiedNearby(&self, unit: BaseNpc, attacker: BaseNpc, event: crate::types::Table) { unimplemented!() }
    pub fn OnHeroLevelUp(&self) { unimplemented!() }
    pub fn OnInventoryContentsChanged(&self) { unimplemented!() }
    pub fn OnItemEquipped(&self, h_item: CBaseItem) { unimplemented!() }
    pub fn OnOwnerDied(&self) { unimplemented!() }
    pub fn OnOwnerSpawned(&self) { unimplemented!() }
    pub fn OnProjectileHit(&self, h_target: Option<BaseNpc>, v_location: Vector) -> bool { unimplemented!() }
    pub fn OnProjectileThink(&self, v_location: Vector) { unimplemented!() }
    pub fn OnSpellStart(&self) { unimplemented!() }
    pub fn OnStolen(&self, h_source_ability: super::ability::CBaseAbility) { unimplemented!() }
    pub fn OnToggle(&self) { unimplemented!() }
    pub fn OnUnStolen(&self) { unimplemented!() }
    pub fn OnUpgrade(&self) { unimplemented!() }
    pub fn ProcsMagicStick(&self) -> bool { unimplemented!() }
    pub fn SpeakTrigger(&self) -> i32 { unimplemented!() }
}
