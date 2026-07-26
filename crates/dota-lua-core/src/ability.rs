use crate::enums;
use crate::types;
use crate::types::event;
use crate::types::event::DeathEvent;
use crate::types::item;
use crate::types::npc;

const DEFAULT_ICON: &str = "";


#[allow(non_snake_case)]
#[allow(unused)]
/// Ability trait (used with `#[ability]`)
/// 
/// ProjectileDT is struct created with `#[derive(ProjectileData)]`
pub trait Ability<ProjectleDT = ()> {
    /// Determine whether an issued command with no target is valid.
    fn CastFilterResult() -> enums::unit_filter::UnitFilterResult { enums::unit_filter::UnitFilterResult::UF_SUCCESS }
    /// Determine whether an issued command on a location is valid.
    fn CastFilterResultLocation(location: types::vector::Vector) -> enums::unit_filter::UnitFilterResult { enums::unit_filter::UnitFilterResult::UF_SUCCESS }
    /// Determine whether an issued command on a target is valid.
    fn CastFilterResultTarget(target: types::npc::BaseNpc) -> enums::unit_filter::UnitFilterResult { enums::unit_filter::UnitFilterResult::UF_SUCCESS }
    /// `-1` level represents current level
    fn GetAbilityChargeRestoreTime(level: i32) -> f64 { 0.0 }
    /// Allows code overriding of the ability texture shown in the HUD.
    fn GetAbilityTextureName() -> String { unimplemented!() }
    /// Controls the size of the AOE casting cursor.
    fn GetAOERadius() -> f64 { 0.0 }
    /// Returns abilities that are stolen simultaneously, or otherwise related in functionality.
    fn GetAssociatedPrimaryAbilities() -> String { unimplemented!() }
    /// Returns other abilities that are stolen simultaneously, or otherwise related in functionality. Generally hidden abilities.
    fn GetAssociatedSecondaryAbilities() -> String { unimplemented!() }
    /// Return cast behavior type of this ability.
    fn GetBehavior() -> enums::ability::DOTA_ABILITY_BEHAVIOR { enums::ability::DOTA_ABILITY_BEHAVIOR::empty() }
    /// Return cast point of this ability (seconds).
    fn GetCastPoint() -> f64 { 0.0 }
    /// Return cast range of this ability.
    fn GetCastRange() -> i32 { 0 }
    /// Return health cost per second of channeling at the given level (-1 is current).
    fn GetChannelledHealthCostPerSecond(level: i32) -> i32 { 0 } // -1 is current level
    /// Return mana cost at the given level per second while channeling (-1 is current).
    fn GetChannelledManaCostPerSecond(level: i32) -> i32 { 0 } // -1 is current level
    /// Return the channel time of this ability.
    fn GetChannelTime() -> f64 { 0.0 }
    /// Return cooldown of this ability.
    fn GetCooldown(level: i32) -> f64 { 0.0 }
    /// Return the error string of a failed command with no target.
    fn GetCustomCastError() -> String { unimplemented!() }
    /// Return the error string of a failed command on a location.
    fn GetCustomCastErrorLocation(location: types::vector::Vector) -> String { unimplemented!() }
    /// Return the error string of a failed command on a target.
    fn GetCustomCastErrorTarget(target: types::npc::BaseNpc) -> String {unimplemented!()}
    /// Return gold cost at the given level (-1 is current).
    fn GetGoldCost(level: i32) -> i32 { 0 }
    /// Return health cost at the given level (-1 is current).
    fn GetHealthCost(level: i32) -> i32 { 0 }
    /// Returns the name of the modifier applied passively by this ability.
    fn GetIntrinsicModifierName() -> &'static str { unimplemented!() }
    /// Return mana cost at the given level (-1 is current).
    fn GetManaCost(level: i32) -> i32 { 0 }
    /// Is this ability an Attribute Bonus.
    fn IsAttributeBonus() -> bool { unimplemented!() }
    /// Returns true if this ability is hidden when stolen by Spell Steal.
    fn IsHiddenWhenStolen() -> bool { unimplemented!() }
    /// Returns true if this ability is refreshed by Refresher Orb.
    fn IsRefreshable() -> bool { true }
    /// Returns true if this ability can be stolen by Spell Steal.
    fn IsStealable() -> bool { true }
    /// Cast time did not complete successfully.
    fn OnAbilityPhaseInterrupted() { unimplemented!() }
    /// Cast time begins (return true for successful cast).
    fn OnAbilityPhaseStart() -> bool { unimplemented!() }
    /// Channel finished.
    fn OnChannelFinish(interrupted: bool) { unimplemented!() }
    /// Channeling is taking place.
    /// 
    /// Interval - seconds
    fn OnChannelThink(interval: f64) { unimplemented!() }
    /// Caster (hero only) gained a level, skilled an ability, or received a new stat bonus.
    fn OnHeroCalculateStatBonus() { unimplemented!() }
    /// A hero has died nearby, takes table of params.
    fn OnHeroDiedNearby(unit: types::npc::BaseNpc, attacker: types::npc::BaseNpc, event: DeathEvent) { unimplemented!() }
    /// Caster gained a level.
    fn OnHeroLevelUp() { unimplemented!() }
    /// Caster inventory changed.
    fn OnInventoryContentsChanged() { unimplemented!() }
    /// Caster equipped item.
    fn OnItemEquipped(item: item::CDOTA_Item) { unimplemented!() }
    /// Caster died.
    fn OnOwnerDied() { unimplemented!() }
    /// Caster respawned or spawned for the first time.
    fn OnOwnerSpawned() { unimplemented!() }
    /// Projectile has collided with a given target or reached its destination. If 'true` is returned, projectile would be destroyed.
    fn OnProjectileHit(target: Option<npc::BaseNpc>, location: types::vector::Vector) -> Option<bool> { unimplemented!() }
    /// Projectile has collided with a given target or reached its destination. If 'true` is returned, projectile would be destroyed.
    /// 
    /// Uses generic type `ProjectileDT` (for more info check [Ability] doc)
    fn OnProjectileHit_ExtraData(target: Option<npc::BaseNpc>, location: types::vector::Vector, extra_data: ProjectleDT) -> Option<bool> { unimplemented!() }

    /// Projectile is actively moving.
    fn OnProjectileThink(location: types::vector::Vector) { unimplemented!() }

    /// Projectile is actively moving.
    /// 
    /// Uses generic type `ProjectileDT` (for more info check [Ability] doc)
    fn OnProjectileThink_ExtraData(location: types::vector::Vector, extra_data: ProjectleDT) { unimplemented!() }

    /// Cast time finished, spell effects begin.
    fn OnSpellStart() { unimplemented!() }
    /// Ability is toggled on/off.
    fn OnToggle() { unimplemented!() }
    /// Ability gained a level.
    fn OnUpgrade() { unimplemented!() }
    fn OtherAbilitiesAlwaysInterruptChanneling() { unimplemented!() }
    /// Returns true if this ability will generate **magic stick** charges for nearby enemies.
    fn ProcsMagicStick() -> bool { true }
    /// Does this ability need the caster to face the target before executing
    fn RequiresFacing() -> bool { unimplemented!() }
    /// Returns true if this ability should return to the default toggle state when its parent respawns.
    fn ResetToggleOnRespawn() -> bool { unimplemented!() }
}