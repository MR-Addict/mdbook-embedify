mod cfg;
mod cli;
mod embed;
mod utils;

use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use std::io;

fn main() {
    let cli = cli::Cli::new();
    let embed = embed::Embed::new();

    // reply --supports command line argument
    cli.reply_supports(&embed);

    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin()).unwrap();
    let result = embed.run(&ctx, book).unwrap();

    // Write the result to stdout
    serde_json::to_writer(io::stdout(), &result).unwrap();
}
