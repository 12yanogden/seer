use clap::Command;

pub trait CommandBuilder {
    fn build(cmd: Command) -> Command;
}
