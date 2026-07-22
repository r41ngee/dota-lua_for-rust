use super::entity::CBaseEntity;
use super::npc::BaseNpc;

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseBuff;

impl CBaseBuff {
    pub fn AddParticle(&self, i: i32, b_destroy_immediately: bool, b_status_effect: bool, i_priority: i32, b_hero_effect: bool, b_overhead_effect: bool) { unimplemented!() }
    pub fn DecrementStackCount(&self) { unimplemented!() }
    pub fn Destroy(&self) { unimplemented!() }
    pub fn ForceRefresh(&self) { unimplemented!() }
    pub fn GetAbility(&self) -> super::ability::CBaseAbility { unimplemented!() }
    pub fn GetAuraDuration(&self) -> f64 { unimplemented!() }
    pub fn GetCaster(&self) -> BaseNpc { unimplemented!() }
    pub fn GetClass(&self) -> String { unimplemented!() }
    pub fn GetCreationTime(&self) -> f64 { unimplemented!() }
    pub fn GetDieTime(&self) -> f64 { unimplemented!() }
    pub fn GetDuration(&self) -> f64 { unimplemented!() }
    pub fn GetElapsedTime(&self) -> f64 { unimplemented!() }
    pub fn GetName(&self) -> String { unimplemented!() }
    pub fn GetParent(&self) -> BaseNpc { unimplemented!() }
    pub fn GetRemainingTime(&self) -> f64 { unimplemented!() }
    pub fn GetStackCount(&self) -> i32 { unimplemented!() }
    pub fn HasFunction(&self, i_function: i32) -> bool { unimplemented!() }
    pub fn IncrementStackCount(&self) { unimplemented!() }
    pub fn IsStunDebuff(&self) -> bool { unimplemented!() }
    pub fn SetDuration(&self, fl_duration: f64, b_inform_client: bool) { unimplemented!() }
    pub fn SetStackCount(&self, i_count: i32) { unimplemented!() }
    pub fn StartIntervalThink(&self, fl_interval: f64) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseModifierLuaHorizontalMotion;

impl CBaseModifierLuaHorizontalMotion {
    pub fn ApplyHorizontalMotionController(&self) -> bool { unimplemented!() }
    pub fn GetPriority(&self) -> i32 { unimplemented!() }
    pub fn OnHorizontalMotionInterrupted(&self) { unimplemented!() }
    pub fn SetPriority(&self, n_motion_priority: i32) { unimplemented!() }
    pub fn UpdateHorizontalMotion(&self, me: BaseNpc, dt: f64) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseModifierLuaMotionBoth;

impl CBaseModifierLuaMotionBoth {
    pub fn ApplyHorizontalMotionController(&self) -> bool { unimplemented!() }
    pub fn ApplyVerticalMotionController(&self) -> bool { unimplemented!() }
    pub fn GetPriority(&self) -> i32 { unimplemented!() }
    pub fn OnHorizontalMotionInterrupted(&self) { unimplemented!() }
    pub fn OnVerticalMotionInterrupted(&self) { unimplemented!() }
    pub fn SetPriority(&self, n_motion_priority: i32) { unimplemented!() }
    pub fn UpdateHorizontalMotion(&self, me: BaseNpc, dt: f64) { unimplemented!() }
    pub fn UpdateVerticalMotion(&self, me: BaseNpc, dt: f64) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseModifierLuaVerticalMotion;

impl CBaseModifierLuaVerticalMotion {
    pub fn ApplyVerticalMotionController(&self) -> bool { unimplemented!() }
    pub fn GetMotionPriority(&self) -> i32 { unimplemented!() }
    pub fn OnVerticalMotionInterrupted(&self) { unimplemented!() }
    pub fn SetMotionPriority(&self, n_motion_priority: i32) { unimplemented!() }
    pub fn UpdateVerticalMotion(&self, me: BaseNpc, dt: f64) { unimplemented!() }
}
