use clap::{Arg, ArgMatches, Command};
use mdbook_preprocessor::Preprocessor;
use std::{
    io::{self, IsTerminal},
    process,
};

pub struct Cli {
    pub matches: ArgMatches,
}

impl Cli {
    pub fn new() -> Self {
        let cmd = Command::new("mdbook-embedify")
            .version(env!("CARGO_PKG_VERSION"))
            .about("A mdbook embed preprocessor that embeds app to your book")
            .subcommand(
                Command::new("supports")
                    .arg(Arg::new("renderer").required(true))
                    .about("Check whether a renderer is supported by this preprocessor"),
            );

        let matches = cmd.clone().get_matches();
        
        // If no subcommand provided and stdin is a terminal, print help
        if matches.subcommand().is_none() && io::stdin().is_terminal() {
            cmd.clone().print_help().unwrap();
            process::exit(1);
        }

        Self { matches }
    }

    pub fn reply_supports(&self, pre: &dyn Preprocessor) {
        if let Some(sub_args) = self.matches.subcommand_matches("supports") {
            // get the renderer
            let renderer = sub_args.get_one::<String>("renderer").unwrap();

            // signal whether the renderer is supported by exiting with 1 or 0.
            match pre.supports_renderer(renderer) {
                Ok(true) => process::exit(0),
                Ok(false) => process::exit(1),
                Err(_) => process::exit(1),
            }
        }
    }
}
