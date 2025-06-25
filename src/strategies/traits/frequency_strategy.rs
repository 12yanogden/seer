use super::strategy::Strategy;

pub trait FrequencyStrategy: Strategy {
    fn matches_frequency(&mut self) -> bool;
    fn is_done(&self) -> bool;
}
