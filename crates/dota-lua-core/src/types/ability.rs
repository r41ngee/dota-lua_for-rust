use super::entity::CBaseEntity;
use crate::{enums, types::{npc, vector}};

/// Struct used to interact with C++ driven abilities
/// 
/// For more info about methods check [crate::ability::Ability]
#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseAbility;

#[allow(non_snake_case)]
impl CBaseAbility {
    pub fn CanAbilityBeUpgraded(&self) -> bool { unimplemented!() }
    pub fn CastAbility(&self) -> bool { unimplemented!() }
    pub fn ContinueCasting(&self) -> bool { unimplemented!() }
    pub fn CreateVisibilityNode(&self, location: super::vector::Vector, radius: f64, duration: f64) { unimplemented!() }
    pub fn DecrementModifierRefCount(&self) { unimplemented!() }
    pub fn EndChannel(&self, interrupted: bool) { unimplemented!() }
    pub fn EndCooldown(&self) { unimplemented!() }
    pub fn ForceSetFrozenCooldown(&self, value: f64) { unimplemented!() }
    pub fn GetAbilityChargeRestoreTime(&self, level: f64) { unimplemented!() }
    pub fn GetAbilityDamage(&self) -> f64 { unimplemented!() }
    pub fn GetAbilityDamageType(&self) -> enums::DAMAGE_TYPE { unimplemented!() }
    pub fn GetAbilityIndex(&self) -> f64 { unimplemented!() }
    /// Returns the name of this ability.
    pub fn GetAbilityName(&self) -> String { unimplemented!() }
    pub fn GetAbilityTargetFlags(&self) -> enums::unit_target::DOTA_UNIT_TARGET_FLAGS { unimplemented!() }
    pub fn GetAbilityTargetTeam(&self) -> enums::unit_target::DOTA_UNIT_TARGET_TEAM { unimplemented!() }
    pub fn GetAbilityTargetType(&self) -> enums::unit_target::DOTA_UNIT_TARGET_TYPE { unimplemented!() }
    pub fn GetAbilityType(&self) -> f64 { unimplemented!() }
    pub fn GetAOERadius(&self) -> f64 { unimplemented!() }
    pub fn GetAssociatedPrimaryAbilities(&self) -> String { unimplemented!() }
    pub fn GetAssociatedSecondaryAbilities(&self) -> String { unimplemented!() }
    pub fn GetAutoCastState(&self) -> bool { unimplemented!() }
    pub fn GetBackswingTime(&self) -> f64 { unimplemented!() }
    pub fn GetBehavior(&self) -> enums::ability::DOTA_ABILITY_BEHAVIOR { unimplemented!() }
    pub fn GetCaster(&self) -> npc::BaseNpc { unimplemented!() }
    pub fn GetCastPoint(&self) -> f64 { unimplemented!() }
    pub fn GetCastRange(&self, location: Option<super::vector::Vector>, target: Option<npc::BaseNpc>) -> f64 { unimplemented!() }
    /// use `-1` for current level
    pub fn GetChannelledHealthCostPerSecond(&self, level: f64) -> f64 { unimplemented!() }
    /// use `-1` for current level
    pub fn GetChannelledManaCostPerSecond(&self, level: f64) -> f64 { unimplemented!() }
    pub fn GetChannelStartTime(&self) -> f64 { unimplemented!() }
    pub fn GetChannelTime(&self) -> f64 { unimplemented!() }
    pub fn GetCloneSource(&self) -> Option<npc::BaseNpc> { unimplemented!() }
    /// use `-1` for current level
    pub fn GetCooldown(&self, level: f64) -> f64 { unimplemented!() }
    pub fn GetCooldownTime(&self) -> f64 { unimplemented!() }
    pub fn GetCooldownTimeRemaining(&self) -> f64 { unimplemented!() }
    pub fn GetCurrentAbilityCharges(&self) -> f64 { unimplemented!() }
    pub fn GetCursorPosition(&self) -> vector::Vector { unimplemented!() }
    pub fn GetCursorTarget(&self) -> Option<npc::BaseNpc> { unimplemented!() }
    pub fn GetCursorTargetingNothing(&self) -> bool { unimplemented!() }
    pub fn GetDuration(&self) -> f64 { unimplemented!() }
    /// use `-1` for current level
    pub fn GetGoldCost(&self, level: f64) -> f64 { unimplemented!() }
    pub fn GetHealthCost(&self, level: f64) -> f64 { unimplemented!() }
    pub fn GetHeroLevelRequiredToUpgrade(&self) -> f64 { unimplemented!() }
    /// use `-1` for current level
    pub fn GetInitialAbilityCharges(&self, level: f64) -> f64 { unimplemented!() }
    pub fn GetIntrinsicModifierName(&self) -> String { unimplemented!() }
    pub fn GetLevel(&self) -> f64 { unimplemented!() }
    /// use `-1` for current level
    pub fn GetLevelSpecialValueFor(&self, name: String, level: f64) { unimplemented!() }
    /// use `-1` for current level
    pub fn GetManaCost(&self, level: f64) -> f64 { unimplemented!() }
    /// use `-1` for current level
    pub fn GetMaxAbilityCharges(&self, level: f64) -> f64 { unimplemented!() }
    pub fn GetMaxLevel(&self) -> f64 { unimplemented!() }
    pub fn GetModifierValue(&self) -> f64 { unimplemented!() }
    pub fn GetModifierValueBonus(&self) -> f64 { unimplemented!() }
    pub fn GetSharedCooldownName(&self) -> String { unimplemented!() }
    pub fn GetSpecialValueFor(&self, name: String) -> f64 { unimplemented!() }
    pub fn GetToggleState(&self) -> bool { unimplemented!() }
    pub fn IncrementModifierRefCount(&self) { unimplemented!() }
    pub fn IsActivated(&self) -> bool { unimplemented!() }
    pub fn IsAttributeBonus(&self) -> bool { unimplemented!() }
    pub fn IsChanneling(&self) -> bool { unimplemented!() }
    pub fn IsCooldownReady(&self) -> bool { unimplemented!() }
    pub fn IsFullyCastable(&self) -> bool { unimplemented!() }
    pub fn IsHidden(&self) -> bool { unimplemented!() }
    pub fn IsHiddenAsSecondaryAbility(&self) -> bool { unimplemented!() }
    pub fn IsHiddenWhenStolen(&self) -> bool { unimplemented!() }
    pub fn IsInAbilityPhase(&self) -> bool { unimplemented!() }
    pub fn IsItem(&self) -> bool { unimplemented!() }
    pub fn IsOwnersManaEnough(&self) -> bool { unimplemented!() }
    pub fn IsPassive(&self) -> bool { unimplemented!() }
    pub fn IsRefreshable(&self) -> bool { unimplemented!() }
    pub fn IsSharedWithTeammates(&self) -> bool { unimplemented!() }
    pub fn IsStealable(&self) -> bool { unimplemented!() }
    pub fn IsStolen(&self) -> bool { unimplemented!() }
    pub fn IsToggle(&self) -> bool { unimplemented!() }
    pub fn IsTrained(&self) -> bool { unimplemented!() }
    pub fn MarkAbilityButtonDirty(&self) -> bool { unimplemented!() }
    pub fn PayGoldCost(&self) { unimplemented!() }
    pub fn PayHealthCost(&self) { unimplemented!() }
    pub fn PayManaCost(&self) { unimplemented!() }
    pub fn ProcsMagicStick(&self) -> bool { unimplemented!() }
    pub fn RefreshCharges(&self) { unimplemented!() }
    pub fn RefreshIntrinsicModifier(&self) { unimplemented!() }
    pub fn RefundHealthCost(&self) { unimplemented!() }
    pub fn RefundManaCost(&self) { unimplemented!() }
    pub fn RequiresFacing(&self) -> bool { unimplemented!() }
    pub fn ResetToggleOnRespawn(&self) -> bool { unimplemented!() }
    pub fn SetActivated(&self, state: bool) { unimplemented!() }
    pub fn SetChanneling(&self, state: bool) { unimplemented!() }
    pub fn SetCurrentAbilityCharges(&self, charges: f64) { unimplemented!() }
    pub fn SetFrozenCooldown(&self, state: bool) { unimplemented!() }
    pub fn SetHidden(&self, state: bool) { unimplemented!() }
    pub fn SetLevel(&self, level: f64) { unimplemented!() }
    pub fn ToggleAbility(&self) { unimplemented!() }
    pub fn ToggleAutoCast(&self) { unimplemented!() }
    pub fn UseResources(&self, useMana: bool, useHealth: bool, gold: bool, cooldown: bool) { unimplemented!() }
}