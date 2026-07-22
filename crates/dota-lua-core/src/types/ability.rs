use super::entity::CBaseEntity;

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseAbility;

#[allow(non_snake_case)]
impl CBaseAbility {
    pub fn Activate(&self) { unimplemented!() }
    pub fn CastFilterResult(&self) -> i32 { unimplemented!() }
    pub fn CastFilterResultLocation(&self, location: super::vector::Vector) -> i32 { unimplemented!() }
    pub fn CastFilterResultTarget(&self, target: CBaseEntity) -> i32 { unimplemented!() }
    pub fn CastAbility(&self) { unimplemented!() }
    pub fn EndCooldown(&self) { unimplemented!() }
    pub fn GetAbilityDamage(&self) -> i32 { unimplemented!() }
    pub fn GetAbilityDamageType(&self) -> i32 { unimplemented!() }
    pub fn GetAbilityName(&self) -> String { unimplemented!() }
    pub fn GetAbilityType(&self) -> i32 { unimplemented!() }
    pub fn GetAnimationIgnoresModelScale(&self) -> bool { unimplemented!() }
    pub fn GetChannelledSpellsPerLevel(&self) -> i32 { unimplemented!() }
    pub fn GetChannelTime(&self) -> f64 { unimplemented!() }
    pub fn GetCaster(&self) -> super::npc::BaseNpc { unimplemented!() }
    pub fn GetCasterRef(&self) -> super::npc::BaseNpc { unimplemented!() }
    pub fn GetCooldown(&self, level: i32) -> f64 { unimplemented!() }
    pub fn GetCooldownTimeRemaining(&self) -> f64 { unimplemented!() }
    pub fn GetCooldownsAreProvisional(&self) -> bool { unimplemented!() }
    pub fn GetEffectiveCooldown(&self, level: i32) -> f64 { unimplemented!() }
    pub fn GetHeroLevelRequiredToUpgrade(&self) -> i32 { unimplemented!() }
    pub fn GetLevel(&self) -> i32 { unimplemented!() }
    pub fn GetMaxLevel(&self) -> i32 { unimplemented!() }
    pub fn GetMinLevel(&self) -> i32 { unimplemented!() }
    pub fn GetManaCost(&self, level: i32) -> f64 { unimplemented!() }
    pub fn GetOverrideCastPoint(&self) -> f64 { unimplemented!() }
    pub fn GetSharedCooldownName(&self) -> String { unimplemented!() }
    pub fn GetSpecialValueFor(&self, key: String) -> f64 { unimplemented!() }
    pub fn GetStunDuration(&self) -> f64 { unimplemented!() }
    pub fn GetToggleState(&self) -> bool { unimplemented!() }
    pub fn IsActivated(&self) -> bool { unimplemented!() }
    pub fn IsHidden(&self) -> bool { unimplemented!() }
    pub fn IsHiddenWhenStolen(&self) -> bool { unimplemented!() }
    pub fn IsCooldownReady(&self) -> bool { unimplemented!() }
    pub fn IsFullyCastable(&self) -> bool { unimplemented!() }
    pub fn IsHiddenAbility(&self) -> bool { unimplemented!() }
    pub fn IsItem(&self) -> bool { unimplemented!() }
    pub fn IsPassive(&self) -> bool { unimplemented!() }
    pub fn IsPermanent(&self) -> bool { unimplemented!() }
    pub fn IsRefreshable(&self) -> bool { unimplemented!() }
    pub fn IsStealable(&self) -> bool { unimplemented!() }
    pub fn IsToggle(&self) -> bool { unimplemented!() }
    pub fn MarkAbilityButtonDirty(&self) { unimplemented!() }
    pub fn OnAbilityEndChannel(&self) { unimplemented!() }
    pub fn OnAbilityStart(&self) { unimplemented!() }
    pub fn OnChannelFinish(&self, bWasCancelled: bool) { unimplemented!() }
    pub fn OnChannelThink(&self, flTime: f64) { unimplemented!() }
    pub fn OnCooldown(&self) -> bool { unimplemented!() }
    pub fn OnHealthPerished(&self) -> bool { unimplemented!() }
    pub fn OnSpellFinish(&self) { unimplemented!() }
    pub fn OnToggle(&self) { unimplemented!() }
    pub fn PayCooldownCost(&self) { unimplemented!() }
    pub fn RefundCooldownCost(&self) { unimplemented!() }
    pub fn SetAbilityIndex(&self, iIndex: i32) { unimplemented!() }
    pub fn SetActivated(&self, bState: bool) { unimplemented!() }
    pub fn SetChannelTime(&self, flTime: f64) { unimplemented!() }
    pub fn SetHidden(&self, bHidden: bool) { unimplemented!() }
    pub fn SetHiddenWhenStolen(&self, bHidden: bool) { unimplemented!() }
    pub fn SetLevel(&self, iLevel: i32) { unimplemented!() }
    pub fn SetOverrideCastPoint(&self, flCastPoint: f64) { unimplemented!() }
    pub fn SetPassive(&self, bState: bool) { unimplemented!() }
    pub fn SetStealable(&self, bState: bool) { unimplemented!() }
    pub fn SetToggle(&self, bState: bool) { unimplemented!() }
    pub fn StartCooldown(&self, flCooldown: f64) { unimplemented!() }
    pub fn ToggleCooldown(&self) { unimplemented!() }
    pub fn UpdateAbilityManacostTable(&self) { unimplemented!() }
    pub fn UseResources(&self, bMana: bool, bCooldown: bool, bOrb: bool, bProcAttack: bool) { unimplemented!() }
}