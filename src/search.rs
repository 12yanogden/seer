use crate::search_strategies::regex::RegexStrategy;
use crate::search_strategies::search_strategy::SearchStrategy;
use crate::search_strategies::target::TargetStrategy;
use clap::ArgMatches;
use std::collections::HashMap;

fn search(matches: &ArgMatches, searchable: String) {
    let mut strategy: Option<Box<dyn SearchStrategy>> = None;
    let mut params = HashMap::new();

    if let Some(target) = matches.get_one::<String>("target") {
        strategy = Some(Box::new(TargetStrategy));
        params.insert(String::from("target"), target.clone());
    } else if let Some(regex) = matches.get_one::<String>("regex") {
        strategy = Some(Box::new(RegexStrategy));
        params.insert(String::from("regex"), regex.clone());
    }

    if let Some(strategy) = strategy {
        let hits = strategy.search(&searchable, &params);
        // Process hits as needed
    }
}
