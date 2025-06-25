use crate::dto::dto::DTO;
use crate::strategies::enums::strategy_type::StrategyType;

pub trait Strategy {
    fn strategy_type(&self) -> StrategyType;
    fn run(&self, dto: &mut DTO);
}
