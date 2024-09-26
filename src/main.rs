use clap::Parser;
use crate::cli::CLI;
use crate::maker::Maker;
mod cli;
mod maker;

#[tokio::main]
async fn main() {
    let cli = CLI::parse();
    let maker = Maker::new();
    maker.make(cli.get_path()).await
}
