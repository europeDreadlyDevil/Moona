use clap::Parser;
use crate::cli::CLI;
use crate::maker::Maker;
mod cli;
mod maker;

#[tokio::main]
async fn main() {
    let cli = CLI::parse();
    let maker = Maker::new();
    if let Some(rule) = cli.get_rule() {
        maker.make_rule(cli.get_path(), rule).await;
    } else {
        maker.make(cli.get_path()).await
    }
}
