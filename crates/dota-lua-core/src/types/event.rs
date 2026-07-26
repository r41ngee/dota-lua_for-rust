/// Representation of death event table of Lua
pub struct DeathEvent;

impl DeathEvent {
    pub fn unit(&self) -> super::npc::BaseNpc { unimplemented!() }
    pub fn attacker(&self) -> super::npc::BaseNpc { unimplemented!() }
    pub fn damage(&self) -> f32 { unimplemented!() }
    pub fn damage_type(&self) -> crate::enums::DAMAGE_TYPE { unimplemented!() }
    pub fn inflictor(&self) -> super::ability::CBaseAbility { unimplemented!() }
}