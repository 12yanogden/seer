#[derive(PartialEq, Debug)]
pub enum StrategyType {
    All,
    Append,
    Count,
    Dir,
    EveryNth,
    File,
    Files,
    FindBetween,
    FindRegex,
    FindString,
    Nth,
    Pipe,
    Prepend,
    ReplaceWith,
    Text,
}
