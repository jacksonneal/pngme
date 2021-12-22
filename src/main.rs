use structopt::StructOpt;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub(crate) type Error = Box<dyn std::error::Error>;
pub(crate) type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = args::Cli::from_args();
    commands::run(cli.subcommand)
}
