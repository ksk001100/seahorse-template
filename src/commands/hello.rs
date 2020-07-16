use crate::actions::hello;
use seahorse::*;

pub fn command() -> Command {
    Command::new("hello")
        .usage("cli hello [args] [flag]: Say hello action")
        .flag(
            Flag::new("bye", FlagType::Bool)
                .usage("--bye, -b: Say bye!")
                .alias("b"),
        )
        .action(hello::action)
}
