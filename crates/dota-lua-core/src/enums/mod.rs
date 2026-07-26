pub mod unit_filter;
pub mod modifier;
pub mod ability;
pub mod unit_target;

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DAMAGE_TYPE {
    DAMAGE_TYPE_NONE = 0,
    DAMAGE_TYPE_PHYSICAL = 1,
    DAMAGE_TYPE_MAGICAL = 2,
    DAMAGE_TYPE_PURE = 4,
    DAMAGE_TYPE_HP_REMOVAL = 8,
}