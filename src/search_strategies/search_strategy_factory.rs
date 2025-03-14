use crate::search_strategies::between::BetweenSearchStrategy;
use crate::search_strategies::regex::RegexSearchStrategy;
use crate::search_strategies::search_strategy::SearchStrategy;
use crate::search_strategies::target::TargetSearchStrategy;
use clap::ArgMatches;
use std::collections::HashMap;

pub struct SearchStrategyFactory;

impl SearchStrategyFactory {
    pub fn create(inputs: &ArgMatches) -> Box<dyn SearchStrategy> {
        if let Some(target) = inputs.get_one::<String>("target") {
            return Box::new(TargetSearchStrategy::new(target.clone()));
        } else if let Some(regex) = inputs.get_one::<String>("regex") {
            return Box::new(RegexSearchStrategy::new(regex.clone()));
        } else if let Some(between) = inputs.get_many::<String>("between") {
            let mut between_iter = between.into_iter();
            let from = between_iter.next().unwrap().clone();
            let to = between_iter.next().unwrap().clone();
            let exclude_inputs = inputs.get_flag("exclude_matches");
            return Box::new(BetweenSearchStrategy::new(from, to, exclude_matches));
        }

        panic!("A search strategy must be provided");
    }
}
