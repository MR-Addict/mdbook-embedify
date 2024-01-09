use clap::{Arg, Command};
use mdbook::preprocess::Preprocessor;
use std::process;

pub fn handle_support_reply(pre: &dyn Preprocessor) {
    let matches = Command::new("mdbook-embedify")
        .about("A mdbook embed preprocessor")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .get_matches();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        let renderer = sub_args
            .get_one::<String>("renderer")
            .expect("Required argument");

        // Signal whether the renderer is supported by exiting with 1 or 0.
        if pre.supports_renderer(renderer) {
            process::exit(0);
        } else {
            process::exit(1);
        }
    }
}
