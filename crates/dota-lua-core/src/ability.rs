use crate::enums;
use crate::types;

#[allow(non_snake_case)]
#[allow(unused)]
pub trait Ability {
    fn CastFilterResult() -> enums::unit_filter::UnitFilterResult { enums::unit_filter::UnitFilterResult::UF_SUCCESS }
    fn CastFilterResultLocation(location: types::vector::Vector) -> enums::unit_filter::UnitFilterResult { enums::unit_filter::UnitFilterResult::UF_SUCCESS }
}