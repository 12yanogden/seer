use crate::hit::Hit;
use crate::search_strategies::search_strategy::SearchStrategy;
use crate::search_strategies::search_strategy_factory::SearchStrategyFactory;
use clap::ArgMatches;
use std::collections::HashMap;

fn search(inputs: &ArgMatches, searchable: String) -> Vec<Hit> {
    let strategy = SearchStrategyFactory::create(inputs);

    let hits = strategy.search(&searchable);
    if inputs.get_one::<u16>("nth").is_some() || inputs.get_one::<u16>("every_nth").is_some() {
        let filtered_hits = apply_frequency(inputs, hits);
        return filtered_hits;
    } else {
        return hits;
    }
}

/// Applies frequency filtering to the hits based on the inputs.
///
/// # Parameters
/// - `inputs`: The command line arguments.
/// - `hits`: The vector of `Hit` structs to filter.
///
/// # Returns
/// A filtered vector of `Hit` structs.
///
/// # Examples
///
/// ```
/// use clap::{ArgMatches, Command};
/// use seek::hit::Hit;
/// use seek::apply_frequency;
///
/// let inputs = Command::new("test")
///     .arg(clap::arg!(--nth <NTH> "Find/replace only the nth match").value_parser(clap::value_parser!(u16)))
///     .try_get_matches_from(vec!["test", "--nth", "1"])
///     .unwrap();
///
/// let hits = vec![
///     Hit::new(String::from("hit1"), 0),
///     Hit::new(String::from("hit2"), 1),
///     Hit::new(String::from("hit3"), 2),
/// ];
///
/// let filtered_hits = apply_frequency(&inputs, hits);
/// assert_eq!(filtered_hits.len(), 1);
/// assert_eq!(filtered_hits[0].get_value(), "hit2");
/// ```
fn apply_frequency(inputs: &ArgMatches, hits: Vec<Hit>) -> Vec<Hit> {
    if let Some(nth) = inputs.get_one::<u16>("nth") {
        hits.into_iter().nth(*nth as usize).into_iter().collect()
    } else if let Some(every_nth) = inputs.get_one::<u16>("every_nth") {
        hits.into_iter()
            .enumerate()
            .filter_map(|(i, hit)| {
                if (i + 1) % *every_nth as usize == 0 {
                    Some(hit)
                } else {
                    None
                }
            })
            .collect()
    } else {
        hits
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Arg, Command};

    fn get_test_inputs(args: Vec<&str>) -> ArgMatches {
        Command::new("test")
            .arg(
                Arg::new("nth")
                    .long("nth")
                    .value_parser(clap::value_parser!(u16)),
            )
            .arg(
                Arg::new("every_nth")
                    .long("every_nth")
                    .value_parser(clap::value_parser!(u16)),
            )
            .try_get_matches_from(args)
            .unwrap()
    }

    #[test]
    fn test_apply_frequency_nth() {
        let inputs = get_test_inputs(vec!["test", "--nth", "1"]);
        let hits = vec![
            Hit::new(String::from("hit1"), 0),
            Hit::new(String::from("hit2"), 1),
            Hit::new(String::from("hit3"), 2),
        ];
        let filtered_hits = apply_frequency(&inputs, hits);
        assert_eq!(filtered_hits.len(), 1);
        assert_eq!(filtered_hits[0].get_value(), "hit2");
    }

    #[test]
    fn test_apply_frequency_every_nth() {
        let inputs = get_test_inputs(vec!["test", "--every_nth", "2"]);
        let hits = vec![
            Hit::new(String::from("hit1"), 0),
            Hit::new(String::from("hit2"), 1),
            Hit::new(String::from("hit3"), 2),
            Hit::new(String::from("hit4"), 3),
        ];
        let filtered_hits = apply_frequency(&inputs, hits);
        assert_eq!(filtered_hits.len(), 2);
        assert_eq!(filtered_hits[0].get_value(), "hit2");
        assert_eq!(filtered_hits[1].get_value(), "hit4");
    }

    #[test]
    fn test_apply_frequency_no_frequency() {
        let inputs = get_test_inputs(vec!["test"]);
        let hits = vec![
            Hit::new(String::from("hit1"), 0),
            Hit::new(String::from("hit2"), 1),
            Hit::new(String::from("hit3"), 2),
        ];
        let filtered_hits = apply_frequency(&inputs, hits);
        assert_eq!(filtered_hits.len(), 3);
    }
}
