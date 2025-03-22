use crate::frequency_strategies::frequency_strategy_factory::FrequencyStrategyFactory;
use crate::hit::Hit;
use crate::search_strategies::search_strategy_factory::SearchStrategyFactory;
use clap::ArgMatches;

pub fn search(inputs: &ArgMatches, searchable: String) -> Vec<Hit> {
    let frequency_strategy = FrequencyStrategyFactory::make(inputs);
    let mut search_strategy = SearchStrategyFactory::make(inputs, frequency_strategy);

    search_strategy.search(&searchable)
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
/// use seek::search::apply_frequency;
///
/// let inputs = Command::new("test")
///     .arg(clap::arg!(--nth <NTH> "Find/replace only the nth match").value_parser(clap::value_parser!(u64)))
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
pub fn apply_frequency(inputs: &ArgMatches, hits: Vec<Hit>) -> Vec<Hit> {
    if let Some(nth) = inputs.get_one::<u64>("nth") {
        hits.into_iter().nth(*nth as usize).into_iter().collect()
    } else if let Some(every_nth) = inputs.get_one::<u64>("every_nth") {
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
                    .value_parser(clap::value_parser!(u64)),
            )
            .arg(
                Arg::new("every_nth")
                    .long("every_nth")
                    .value_parser(clap::value_parser!(u64)),
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
