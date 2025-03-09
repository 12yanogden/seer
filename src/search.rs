use crate::search_strategies::between::BetweenSearchStrategy;
use crate::search_strategies::regex::RegexSearchStrategy;
use crate::search_strategies::search_strategy::SearchStrategy;
use crate::search_strategies::target::TargetSearchStrategy;
use clap::ArgMatches;
use std::collections::HashMap;

fn search(matches: &ArgMatches, searchable: String) {
    let mut strategy: Option<Box<dyn SearchStrategy>> = None;
    let mut params = HashMap::new();

    if let Some(target) = matches.get_one::<String>("target") {
        strategy = Some(Box::new(TargetSearchStrategy));
        params.insert(String::from("target"), target.clone());
    } else if let Some(regex) = matches.get_one::<String>("regex") {
        strategy = Some(Box::new(RegexSearchStrategy));
        params.insert(String::from("regex"), regex.clone());
    } else if let Some(between) = matches.get_many::<String>("between") {
        let mut between_iter = between.into_iter();
        let from = between_iter.next().unwrap().clone();
        let to = between_iter.next().unwrap().clone();
        strategy = Some(Box::new(BetweenSearchStrategy));
        params.insert(String::from("from"), from);
        params.insert(String::from("to"), to);
        if matches.get_flag("exclude_matches") {
            params.insert(String::from("exclude_matches"), String::from("true"));
        }
    }

    if let Some(strategy) = strategy {
        let hits = strategy.search(&searchable, &params);
        // Process hits as needed
    }
}
