mod actions;
mod commands;

use commands::hello;
use seahorse::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage(format!("{} [command] [args]", env!("CARGO_PKG_NAME")))
        .command(hello::command());

    app.run(args);
}
